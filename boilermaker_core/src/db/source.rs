use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::{Result, eyre::eyre};
use tabled::Tabled;
use unicode_truncate::{Alignment, UnicodeTruncateStr};

use crate::template as tmpl;
use crate::util::crypto::sha256_hash_string;
use crate::util::file::read_file_to_string;

use super::LocalCache;

#[async_trait::async_trait]
pub trait SourceMethods: Send + Sync {
    async fn add_source(
        &self,
        source_row: SourceRow,
        partial_source_template_rows: Vec<(PathBuf, PartialSourceTemplateRow)>,
    ) -> Result<AddSourceResult>;
    async fn list_sources(&self) -> Result<Vec<SourceRow>>;
    //async fn search_sources(&self, term: &str) -> Result<Vec<SourceRow>>;
}

#[async_trait::async_trait]
impl SourceMethods for LocalCache {
    // TODO: split up 3 types of queries into separate functions for readability
    #[tracing::instrument]
    async fn add_source(
        &self,
        source_row: SourceRow,
        partial_source_template_rows: Vec<(PathBuf, PartialSourceTemplateRow)>,
    ) -> Result<AddSourceResult> {
        let mut tx = self.pool.begin().await?;

        let source_result = sqlx::query(
            r#"
            INSERT INTO source
              (name, backend, coordinate, description, sha256_hash, created_at)
            VALUES
              (?, ?, ?, ?, ?, strftime('%s','now'));
            "#,
        )
        .bind(&source_row.name)
        .bind(&source_row.backend)
        .bind(&source_row.coordinate)
        .bind(&source_row.description)
        .bind(&source_row.sha256_hash)
        .execute(&mut *tx)
        .await?;

        let source_id = source_result.last_insert_rowid();

        let mut source_template_ids: Vec<i64> = Vec::new();
        for (path, partial) in partial_source_template_rows.into_iter() {
            let source_template_row = SourceTemplateRow {
                source_id,
                repo: partial.repo,
                lang: partial.lang,
                name: partial.name,
                branch: partial.branch,
                subdir: partial.subdir,
                sha256_hash: None,
            }
            .set_hash_string();

            let template_result = sqlx::query(
                r#"
                INSERT INTO source_template
                  (source_id, repo, lang, name, branch, subdir, sha256_hash, created_at)
                VALUES
                  (?, ?, ?, ?, ?, ?, ?, strftime('%s','now'));
                "#,
            )
            .bind(source_id)
            .bind(&source_template_row.repo)
            .bind(&source_template_row.lang)
            .bind(&source_template_row.name)
            .bind(&source_template_row.branch)
            .bind(&source_template_row.subdir)
            .bind(&source_template_row.sha256_hash)
            .execute(&mut *tx)
            .await?;

            let source_template_id = template_result.last_insert_rowid();

            let files = tmpl::list_template_files(&path).await?;
            for file in files {
                let content = read_file_to_string(&file)?;
                let _ = sqlx::query(
                    r#"
                    INSERT INTO source_template_content
                      (source_template_id, file_path, content, created_at)
                    VALUES
                      (?, ?, ?, strftime('%s','now'));
                    "#,
                )
                .bind(source_template_id)
                .bind(file.to_string_lossy().to_string())
                .bind(content)
                .execute(&mut *tx)
                .await?;
            }

            source_template_ids.push(source_template_id);
        }

        tx.commit().await?;

        Ok(AddSourceResult {
            source_id,
            source_template_ids,
        })
    }

    #[tracing::instrument]
    async fn list_sources(&self) -> Result<Vec<SourceRow>> {
        let results = sqlx::query_as::<_, SourceRow>(
            r#"
                SELECT name,
                       backend,
                       coordinate,
                       description,
                       sha256_hash
                FROM source
                ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(results)
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SourceRow {
    pub name: String,
    pub backend: String,
    pub coordinate: String,
    pub description: Option<String>,
    pub sha256_hash: Option<String>,
}

impl SourceRow {
    #[tracing::instrument]
    pub fn set_hash_string(mut self) -> Self {
        let hash = hash_source_row(&self);
        self.sha256_hash = Some(hash);
        self
    }
}

pub fn hash_source_row(row: &SourceRow) -> String {
    let input = format!("{}~~{}~~{}", row.name, row.backend, row.coordinate);
    sha256_hash_string(&input)
}

#[derive(Debug, Clone)]
pub struct PartialSourceTemplateRow {
    pub repo: String,
    pub lang: String,
    pub name: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SourceTemplateRow {
    pub source_id: i64,
    pub repo: String,
    pub lang: String,
    pub name: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub sha256_hash: Option<String>,
}

// TODO: increase validation
pub fn hashmap_into_source_template_row(
    source_id: i64,
    m: &HashMap<String, String>,
) -> Result<SourceTemplateRow> {
    let repo = m
        .get("repo")
        .cloned()
        .ok_or(eyre!("Template missing repo"))?;
    let url = repo.clone();
    let lang = m
        .get("lang")
        .cloned()
        .ok_or(eyre!("Template missing lang"))?;

    let mut row = SourceTemplateRow {
        source_id,
        repo,
        lang,
        name: tmpl::make_name_from_url(&url),
        branch: m.get("branch").cloned(),
        subdir: m.get("subdir").cloned(),
        sha256_hash: None,
    };
    row = row.set_hash_string();

    Ok(row)
}

impl SourceTemplateRow {
    #[tracing::instrument]
    pub fn set_hash_string(mut self) -> Self {
        let hash = hash_source_template_row(&self);
        self.sha256_hash = Some(hash);
        self
    }
}

// TODO: merge with hash_template_row (possibly via trait/impl or shared struct)
pub fn hash_source_template_row(row: &SourceTemplateRow) -> String {
    let input = format!(
        "{}~~{}~~{}~~{}~~{}",
        row.repo,
        row.name,
        row.lang,
        row.branch.as_deref().unwrap_or(""),
        row.subdir.as_deref().unwrap_or(""),
    );
    sha256_hash_string(&input)
}

#[derive(Debug, Clone)]
pub struct AddSourceResult {
    pub source_id: i64,
    pub source_template_ids: Vec<i64>,
}

#[derive(Debug, Tabled)]
pub struct TabledSourceRow {
    pub name: String,
    pub backend: String,
    pub coordinate: String,
    pub description: String,
}

impl TabledSourceRow {
    pub fn from(row: SourceRow) -> Self {
        let coordinate = row.coordinate.unicode_pad(80, Alignment::Left, true);
        Self {
            name: row.name,
            backend: row.backend,
            description: row.description.unwrap_or("-".to_string()),
            coordinate: coordinate.to_string(),
        }
    }
}
