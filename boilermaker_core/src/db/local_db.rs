use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::{eyre::eyre, Result};
use sqlx::{
    QueryBuilder, migrate::Migrator, sqlite::{SqliteConnectOptions, SqlitePool}
};
use tabled::Tabled;

use crate::template as tmpl;
use crate::util::crypto::sha256_hash_string;
use crate::util::file::read_file_to_string;
use crate::util::time::timestamp_to_iso8601;

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[async_trait::async_trait]
pub trait TemplateDb: Send + Sync {
    // ................................................. Schema
    // TODO: rename create_schema or similar (now local_db has cache + sources)
    async fn create_schema(&self) -> Result<()>;
    // ................................................. Templates
    async fn check_unique(&self, row: &TemplateRow) -> Result<Option<TemplateResult>>;
    async fn create_template(&self, row: TemplateRow) -> Result<i64>;
    async fn delete_template(&self, id: i64) -> Result<i64>;
    async fn find_templates(&self, query: TemplateFindParams) -> Result<Vec<TemplateResult>>;
    async fn get_template(&self, id: i64) -> Result<Option<TemplateResult>>;
    async fn index_template(&self, id: i64) -> Result<()>;
    async fn list_templates(
        &self,
        opts: Option<ListTemplateOptions>,
    ) -> Result<Vec<TemplateResult>>;
    async fn search_templates(&self, term: &str) -> Result<Vec<SearchResult>>;
    async fn template_table_exists(&self) -> Result<bool>;
    async fn update_template(&self, id: i64, row: TemplateRow) -> Result<i64>;
    // ................................................. Sources
    async fn add_source(
        &self,
        source_row: SourceRow,
        partial_source_template_rows: Vec<(PathBuf, PartialSourceTemplateRow)>,
    ) -> Result<AddSourceResult>;
}

#[derive(Debug)]
pub struct LocalCache {
    pub pool: SqlitePool,
    pub path: String,
}

impl LocalCache {
    #[tracing::instrument]
    pub async fn new(db_path: &str) -> Result<Self> {
        let options = SqliteConnectOptions::new()
            .filename(db_path)
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await?;
        Ok(Self {
            pool,
            path: db_path.to_owned(),
        })
    }
}

#[async_trait::async_trait]
impl TemplateDb for LocalCache {
    #[tracing::instrument]
    async fn check_unique(&self, row: &TemplateRow) -> Result<Option<TemplateResult>> {
        let result = sqlx::query_as::<_, TemplateResult>(
            r#"
            SELECT *
            FROM template
            WHERE
              name = ?1 AND
              lang = ?2 AND
              repo = ?3;
            "#,
        )
        .bind(&row.name)
        .bind(&row.lang)
        .bind(&row.repo)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    #[tracing::instrument]
    async fn create_template(&self, row: TemplateRow) -> Result<i64> {
        let template_result = sqlx::query(
            r#"
            INSERT INTO template
              (name, lang, template_dir, created_at, repo, branch, subdir, sha256_hash)
            VALUES
              (?, ?, ?, strftime('%s','now'), ?, ?, ?, ?);
            "#,
        )
        .bind(&row.name)
        .bind(&row.lang)
        .bind(&row.template_dir)
        .bind(&row.repo)
        .bind(&row.branch)
        .bind(&row.subdir)
        .bind(&row.sha256_hash)
        .execute(&self.pool)
        .await?;

        Ok(template_result.last_insert_rowid())
    }

    #[tracing::instrument]
    async fn create_schema(&self) -> Result<()> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }

    #[tracing::instrument]
    async fn delete_template(&self, id: i64) -> Result<i64> {
        let _result = sqlx::query("DELETE FROM template WHERE id = ?;")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(id)
    }

    //TODO: add regexs, fuzzy matching, predicates, etc
    #[tracing::instrument]
    async fn find_templates(&self, params: TemplateFindParams) -> Result<Vec<TemplateResult>> {
        let mut qb = QueryBuilder::new("SELECT * FROM template WHERE 1=1");

        if let Some(ids) = params.ids && !ids.is_empty() {
            qb.push(" AND id IN (");
            let mut separated = qb.separated(",");
            for id in ids {
                separated.push_bind(id);
            }
            separated.push_unseparated(")");
        }
        if let Some(name) = params.name {
            qb.push(" AND name = ?");
            qb.push_bind(name);
        }
        if let Some(lang) = params.lang {
            qb.push(" AND lang = ?");
            qb.push_bind(lang);
        }
        if let Some(repo) = params.repo {
            qb.push(" AND repo = ?");
            qb.push_bind(repo);
        }
        if let Some(branch) = params.branch {
            qb.push(" AND branch = ?");
            qb.push_bind(branch);
        }
        if let Some(subdir) = params.subdir {
            qb.push(" AND subdir = ?");
            qb.push_bind(subdir);
        }
        qb.push(" ORDER BY name ASC");
        let q = qb.build_query_as::<TemplateResult>();
        let results = q.fetch_all(&self.pool).await?;

        Ok(results)
    }

    #[tracing::instrument]
    async fn index_template(&self, id: i64) -> Result<()> {
        let t = self
            .get_template(id)
            .await?
            .ok_or_else(|| eyre!("Template with id {} not found", id))?;

        let files = tmpl::list_template_files(&PathBuf::from(&t.template_dir)).await?;
        for file in files {
            let content = read_file_to_string(&file)?;
            let _ = sqlx::query(
                r#"
                INSERT INTO template_content
                  (template_id, file_path, content, created_at)
                VALUES
                  (?, ?, ?, strftime('%s','now'));
                "#,
            )
            .bind(id)
            .bind(file.to_string_lossy().to_string())
            .bind(content)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    // TODO: add options for ordering, pagination, filtering, etc
    #[tracing::instrument]
    async fn list_templates(
        &self,
        _opts: Option<ListTemplateOptions>,
    ) -> Result<Vec<TemplateResult>> {
        let results =
            sqlx::query_as::<_, TemplateResult>("SELECT * FROM template ORDER BY created_at DESC;")
                .fetch_all(&self.pool)
                .await?;

        Ok(results)
    }

    #[tracing::instrument]
    async fn get_template(&self, id: i64) -> Result<Option<TemplateResult>> {
        let result = sqlx::query_as::<_, TemplateResult>("SELECT * FROM template WHERE id = ?;")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result)
    }

    #[tracing::instrument]
    async fn search_templates(&self, term: &str) -> Result<Vec<SearchResult>> {
        let term = term.trim();
        let results = sqlx::query_as::<_, SearchResult>(
            r#"
            SELECT
              src.template_id,
              t.name, t.lang, t.template_dir, t.repo, t.branch, t.subdir,
              t.created_at, t.updated_at, t.sha256_hash,
              src.file_path, src.content
            FROM template_content_fts AS ft_search
              LEFT JOIN template_content AS src ON ft_search.rowid=src.id
              LEFT JOIN template as t ON src.template_id=t.id
            WHERE
              template_content_fts MATCH ?
            "#,
        )
        .bind(term)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    #[tracing::instrument]
    async fn template_table_exists(&self) -> Result<bool> {
        // TODO: rewrite with compile-time macros in sqlx
        let row: (i64,) = sqlx::query_as(
            "SELECT COUNT(name) FROM sqlite_master WHERE type='table' AND name='template';",
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0 > 0)
    }

    #[tracing::instrument]
    async fn update_template(&self, id: i64, row: TemplateRow) -> Result<i64> {
        let _ = sqlx::query(
            r#"
            UPDATE template
            SET name = ?,
                lang = ?,
                template_dir = ?,
                repo = ?,
                branch = ?,
                subdir = ?,
                sha256_hash = ?,
                updated_at = unixepoch()
            WHERE id = ?
            RETURNING id;
            "#,
        )
        .bind(row.name)
        .bind(row.lang)
        .bind(row.template_dir)
        .bind(row.repo)
        .bind(row.branch)
        .bind(row.subdir)
        .bind(row.sha256_hash)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

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
}

#[derive(Debug, Clone)]
pub struct TemplateRow {
    pub name: String,
    pub lang: String,
    pub template_dir: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub sha256_hash: Option<String>,
}

impl TemplateRow {
    #[tracing::instrument]
    pub fn set_hash_string(mut self) -> Self {
        let hash = hash_template_row(&self);
        self.sha256_hash = Some(hash);
        self
    }
}

impl From<TemplateResult> for TemplateRow {
    fn from(value: TemplateResult) -> Self {
        TemplateRow {
            name: value.name,
            lang: value.lang,
            template_dir: value.template_dir,
            repo: value.repo,
            branch: value.branch,
            subdir: value.subdir,
            sha256_hash: value.sha256_hash,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TemplateResult {
    pub id: i64,
    pub name: String,
    pub lang: String,
    pub template_dir: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub sha256_hash: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct TemplateFindParams {
    pub ids: Option<Vec<i64>>,
    pub name: Option<String>,
    pub lang: Option<String>,
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub sha256_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListTemplateOptions {
    pub order_by: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Debug, Tabled)]
pub struct DisplayableTemplateListResult {
    pub id: i64,
    pub name: String,
    pub lang: String,
    pub repo: String,
    pub created_at: String,
    pub updated_at: String,
}

impl DisplayableTemplateListResult {
    pub fn to_std_row(row: TemplateResult) -> Self {
        Self {
            id: row.id,
            name: row.name,
            lang: row.lang,
            repo: row.repo,
            created_at: row
                .created_at
                .map(|v| timestamp_to_iso8601(v as i64))
                .unwrap_or_else(|| "-".to_string()),
            updated_at: row
                .updated_at
                .map(|v| timestamp_to_iso8601(v as i64))
                .unwrap_or_else(|| "-".to_string()),
        }
    }
}

pub fn hash_template_row(row: &TemplateRow) -> String {
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

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SearchResult {
    pub template_id: i64,
    pub name: String,
    pub lang: String,
    pub template_dir: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
    pub sha256_hash: Option<String>,
    pub file_path: String,
    pub content: String,
}

#[derive(Debug, Clone)]
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
