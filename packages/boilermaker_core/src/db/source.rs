use std::path::PathBuf;

use color_eyre::Result;
use serde::Serialize;
use sqlx::QueryBuilder;
use tabled::Tabled;
use unicode_truncate::{Alignment, UnicodeTruncateStr};

use crate::{
    db::template::ListTemplateOptions,
    db::{SearchOptions, SearchResult},
    template as tmpl,
    template::{InstallableTemplate, make_name_from_url},
    util::crypto::sha256_hash_string,
    util::file::read_file_to_string,
};

use super::LocalDb;

type SourceId = i64;
type SourceTemplateId = i64;

#[async_trait::async_trait]
pub trait SourceMethods: Send + Sync {
    async fn add_source(
        &self,
        source_row: SourceRow,
        partial_source_template_rows: Vec<(PathBuf, PartialSourceTemplateRow)>,
    ) -> Result<AddSourceResult>;
    async fn find_alt_lang_impls(
        &self,
        source_template: &SourceTemplateResult,
    ) -> Result<Vec<SourceTemplateResult>>;
    async fn find_sources(&self, query: SourceFindParams) -> Result<Vec<SourceResult>>;
    async fn find_source_templates(
        &self,
        query: SourceTemplateFindParams,
    ) -> Result<Vec<SourceTemplateResult>>;
    async fn get_source_template(
        &self,
        source_template_id: SourceTemplateId,
    ) -> Result<Option<SourceTemplateResult>>;
    // TODO: add get_source_template_content for a single file's contents
    async fn get_source_template_content_all(
        &self,
        source_id: SourceId,
    ) -> Result<Vec<SourceTemplateContentResult>>;
    async fn get_source_template_content_readme_boilermaker(
        &self,
        source_template_id: SourceTemplateId,
    ) -> Result<SourceTemplateContentReadmeBoilermaker>;
    async fn list_sources(&self) -> Result<Vec<SourceResult>>;
    async fn list_source_templates(
        &self,
        source_id: SourceId,
        opts: Option<&ListTemplateOptions>,
    ) -> Result<Vec<SourceTemplateResult>>;
    async fn search_sources(
        &self,
        source_name: Option<String>,
        term: &str,
        opts: Option<SearchOptions>,
    ) -> Result<Vec<SearchResult>>;
}

#[async_trait::async_trait]
impl SourceMethods for LocalDb {
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
              (name, backend, coordinate, description, sha256_hash, created_at, readme)
            VALUES
              (?, ?, ?, ?, ?, strftime('%s','now'), ?);
            "#,
        )
        .bind(&source_row.name)
        .bind(&source_row.backend)
        .bind(&source_row.coordinate)
        .bind(&source_row.description)
        .bind(&source_row.sha256_hash)
        .bind(&source_row.readme)
        .execute(&mut *tx)
        .await?;

        let source_id = source_result.last_insert_rowid();

        let mut source_template_ids: Vec<SourceTemplateId> = Vec::new();
        for (path, partial) in partial_source_template_rows.into_iter() {
            let source_template_row = SourceTemplateRow {
                source_id,
                repo: partial.repo,
                lang: partial.lang,
                name: partial.name,
                branch: partial.branch,
                subdir: partial.subdir,
                config: partial.config,
                sha256_hash: None,
            }
            .set_hash_string();

            let template_result = sqlx::query(
                r#"
                INSERT INTO source_template
                  (source_id, repo, lang, name, branch, subdir, sha256_hash, created_at, config)
                VALUES
                  (?, ?, ?, ?, ?, ?, ?, strftime('%s','now'), ?);
                "#,
            )
            .bind(source_id)
            .bind(&source_template_row.repo)
            .bind(&source_template_row.lang)
            .bind(&source_template_row.name)
            .bind(&source_template_row.branch)
            .bind(&source_template_row.subdir)
            .bind(&source_template_row.sha256_hash)
            .bind(&source_template_row.config)
            .execute(&mut *tx)
            .await?;

            let source_template_id = template_result.last_insert_rowid();

            let repo_name_relative = make_name_from_url(&source_template_row.repo);

            let files = tmpl::list_template_files(&path).await?;
            for file in files {
                let file_path = file.to_string_lossy().to_string();
                let base_path_index = file_path.find(&repo_name_relative).unwrap();
                let file_path = file_path
                    .split_at(base_path_index)
                    .1
                    .replace(&repo_name_relative, "");

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
                .bind(file_path)
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

    /// Return Source Templates where repo/branch/subdir match but lang does not match lang.
    #[tracing::instrument]
    async fn find_alt_lang_impls(
        &self,
        source_template: &SourceTemplateResult,
    ) -> Result<Vec<SourceTemplateResult>> {
        let mut qb = QueryBuilder::new("SELECT * FROM source_template WHERE 1=1");

        qb.push(" AND repo = ");
        qb.push_bind(&source_template.repo);

        qb.push(" AND lang != ");
        qb.push_bind(&source_template.lang);

        if let Some(branch) = &source_template.branch {
            qb.push(" AND branch = ");
            qb.push_bind(branch);
        } else {
            qb.push(" AND branch IS NULL");
        }

        if let Some(subdir) = &source_template.subdir {
            qb.push(" AND subdir = ");
            qb.push_bind(subdir);
        } else {
            qb.push(" AND subdir IS NULL");
        }

        qb.push(" ORDER BY name ASC");

        let q = qb.build_query_as::<SourceTemplateResult>();
        let results = q.fetch_all(&self.pool).await?;

        Ok(results)
    }

    //TODO: add regexs, fuzzy matching, predicates, etc
    #[tracing::instrument]
    async fn find_sources(&self, params: SourceFindParams) -> Result<Vec<SourceResult>> {
        let mut qb = QueryBuilder::new("SELECT * FROM source WHERE 1=1");

        /*
        if let Some(ids) = params.ids
            && !ids.is_empty()
        {
            qb.push(" AND id IN (");
            let mut separated = qb.separated(",");
            for id in ids {
                separated.push_bind(id);
            }
            separated.push_unseparated(")");
        }
         */
        if let Some(name) = params.name {
            qb.push(" AND name = ");
            qb.push_bind(name);
        }
        if let Some(coordinate) = params.coordinate {
            qb.push(" AND coordinate = ");
            qb.push_bind(coordinate);
        }
        if let Some(description) = params.description {
            qb.push(" AND description = ");
            qb.push_bind(description);
        }
        if let Some(sha256_hash) = params.sha256_hash {
            qb.push(" AND sha256_hash = ");
            qb.push_bind(sha256_hash);
        }
        qb.push(" ORDER BY name ASC");

        let q = qb.build_query_as::<SourceResult>();
        let results = q.fetch_all(&self.pool).await?;

        Ok(results)
    }

    //TODO: add regexs, fuzzy matching, predicates, etc
    #[tracing::instrument]
    async fn find_source_templates(
        &self,
        params: SourceTemplateFindParams,
    ) -> Result<Vec<SourceTemplateResult>> {
        let mut qb = QueryBuilder::new("SELECT * FROM source_template WHERE 1=1");

        /*
        if let Some(ids) = params.ids
            && !ids.is_empty()
        {
            qb.push(" AND id IN (");
            let mut separated = qb.separated(",");
            for id in ids {
                separated.push_bind(id);
            }
            separated.push_unseparated(")");
        }
         */
        // TODO: add source_ids vec

        if let Some(name) = params.name {
            qb.push(" AND name = ");
            qb.push_bind(name);
        }
        if let Some(lang) = params.lang {
            qb.push(" AND lang = ");
            qb.push_bind(lang);
        }
        if let Some(repo) = params.repo {
            qb.push(" AND repo = ");
            qb.push_bind(repo);
        }
        if let Some(branch) = params.branch {
            qb.push(" AND branch = ");
            qb.push_bind(branch);
        }
        if let Some(subdir) = params.subdir {
            qb.push(" AND subdir = ");
            qb.push_bind(subdir);
        }
        qb.push(" ORDER BY name ASC");

        let q = qb.build_query_as::<SourceTemplateResult>();
        let results = q.fetch_all(&self.pool).await?;

        Ok(results)
    }

    #[tracing::instrument]
    async fn get_source_template(
        &self,
        source_template_id: SourceTemplateId,
    ) -> Result<Option<SourceTemplateResult>> {
        let result = sqlx::query_as::<_, SourceTemplateResult>(
            "SELECT * FROM source_template WHERE id = ?;",
        )
        .bind(source_template_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    // TODO: set relative (or at least useful) paths for file.file_path (not /var/tmp/...) when
    // inserting contents
    #[tracing::instrument]
    async fn get_source_template_content_all(
        &self,
        source_id: SourceId,
    ) -> Result<Vec<SourceTemplateContentResult>> {
        let results = sqlx::query_as::<_, SourceTemplateContentResult>(
            "SELECT * FROM source_template_content WHERE source_template_id = ?;",
        )
        .bind(source_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(results)
    }

    #[tracing::instrument]
    async fn get_source_template_content_readme_boilermaker(
        &self,
        source_template_id: SourceTemplateId,
    ) -> Result<SourceTemplateContentReadmeBoilermaker> {
        let results = sqlx::query_as::<_, SourceTemplateContentResult>(
            r#"
                SELECT *
                FROM source_template_content
                WHERE
                    source_template_id = ?
                    AND (file_path LIKE '%/README.%' OR file_path LIKE '/boilermaker.%')
                COLLATE NOCASE
            "#,
        )
        .bind(source_template_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(SourceTemplateContentReadmeBoilermaker::from(results))
    }

    #[tracing::instrument]
    async fn list_sources(&self) -> Result<Vec<SourceResult>> {
        let results = sqlx::query_as::<_, SourceResult>(
            r#"
                SELECT id,
                       name,
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

    #[tracing::instrument]
    async fn list_source_templates(
        &self,
        source_id: SourceId,
        options: Option<&ListTemplateOptions>,
    ) -> Result<Vec<SourceTemplateResult>> {
        let results = match options {
            None => {
                sqlx::query_as::<_, SourceTemplateResult>(
                    "SELECT * FROM source_template WHERE source_id = ? ORDER BY name ASC LIMIT 50",
                )
                .bind(source_id)
                .fetch_all(&self.pool)
                .await?
            }
            Some(opts) => {
                let mut qb = QueryBuilder::new("SELECT * FROM source_template WHERE source_id = ");
                qb.push_bind(source_id);

                if let Some(order_by) = &opts.order_by {
                    qb.push(format!(" ORDER BY {order_by} "));
                } else {
                    qb.push(" ORDER BY name ASC");
                }

                if let Some(limit) = opts.limit {
                    qb.push(" LIMIT ");
                    qb.push_bind(limit);
                } else {
                    qb.push(" LIMIT 50");
                }

                if let Some(offset) = opts.offset {
                    qb.push(" OFFSET ");
                    qb.push_bind(offset);
                }

                let q = qb.build_query_as::<SourceTemplateResult>();
                q.fetch_all(&self.pool).await?
            }
        };

        Ok(results)
    }

    // Search the content of all templates in source_name.
    #[tracing::instrument]
    async fn search_sources(
        &self,
        source_name: Option<String>,
        term: &str,
        opts: Option<SearchOptions>,
    ) -> Result<Vec<SearchResult>> {
        let term = term.trim();
        let include_content = if let Some(opts) = opts {
            opts.content
        } else {
            false
        };

        let select_stmt = if include_content {
            r#"
            SELECT
                'source' AS kind,
                st.id, st.name, st.lang, st.repo, st.branch, st.subdir,
                ft_search.content
            "#
        } else {
            r#"
            SELECT
                'source' AS kind,
                st.id, st.name, st.lang, st.repo, st.branch, st.subdir,
                '' AS content
            "#
        };

        let mut qb = QueryBuilder::new(format!(
            r#"
            {select_stmt}
            FROM source_template_content_fts AS ft_search
                LEFT JOIN source_template_content AS stc ON ft_search.rowid = stc.id
                LEFT JOIN source_template as st ON stc.source_template_id = st.id
                LEFT JOIN source as s ON st.source_id = s.id
            WHERE source_template_content_fts MATCH
            "#
        ));
        qb.push_bind(term);

        if let Some(name) = source_name {
            qb.push(" AND s.name = ");
            qb.push_bind(name);
        }
        qb.push(" GROUP BY st.id");

        qb.push(" ORDER BY rank DESC");

        let q = qb.build_query_as::<SearchResult>();
        Ok(q.fetch_all(&self.pool).await?)
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SourceRow {
    pub name: String,
    pub backend: String,
    pub coordinate: String,
    pub description: Option<String>,
    pub sha256_hash: Option<String>,
    pub readme: Option<String>,
}

impl SourceRow {
    #[tracing::instrument]
    pub fn set_hash_string(mut self) -> Self {
        let hash = hash_source_row(&self);
        self.sha256_hash = Some(hash);
        self
    }
}

// TODO: is this cruft?
pub fn hash_source_row(row: &SourceRow) -> String {
    let input = format!("{}~~{}~~{}", row.name, row.backend, row.coordinate);
    sha256_hash_string(&input)
}

#[derive(Debug, Clone)]
pub struct PartialSourceTemplateRow {
    pub repo: String,
    pub lang: String,
    pub name: String,
    pub config: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SourceTemplateRow {
    pub source_id: SourceId,
    pub repo: String,
    pub lang: String,
    pub name: String,
    pub config: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub sha256_hash: Option<String>,
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
    pub source_id: SourceId,
    pub source_template_ids: Vec<SourceTemplateId>,
}

#[derive(Debug, Tabled)]
pub struct TabledSourceRow {
    pub id: SourceId,
    pub name: String,
    //pub coordinate: String,
    pub description: String,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct SourceResult {
    pub id: SourceId,
    pub name: String,
    pub backend: String,
    pub coordinate: String,
    pub description: Option<String>,
    pub sha256_hash: Option<String>,
}

impl TabledSourceRow {
    pub fn from(row: SourceResult) -> Self {
        let description = match row.description {
            None => "-".to_string(),
            Some(s) => {
                if s.len() >= 33 {
                    s.unicode_pad(33, Alignment::Left, true).to_string()
                } else {
                    s
                }
            }
        };

        Self {
            id: row.id,
            name: row.name,
            description,
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct SourceTemplateResult {
    pub id: SourceTemplateId,
    pub source_id: SourceId,
    pub name: String,
    pub lang: String,
    pub repo: String,
    pub config: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub sha256_hash: Option<String>,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
}

impl InstallableTemplate for SourceTemplateResult {
    fn id(&self) -> i64 {
        self.id
    }

    fn repo(&self) -> &str {
        &self.repo
    }

    fn lang(&self) -> Option<&String> {
        Some(&self.lang)
    }

    fn branch(&self) -> Option<&String> {
        self.branch.as_ref()
    }

    fn subdir(&self) -> Option<&String> {
        self.subdir.as_ref()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SourceFindParams {
    pub ids: Option<Vec<SourceId>>,
    pub name: Option<String>,
    pub coordinate: Option<String>,
    pub description: Option<String>,
    pub sha256_hash: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SourceTemplateFindParams {
    pub ids: Option<Vec<SourceTemplateId>>,
    pub source_ids: Option<Vec<SourceId>>,
    pub name: Option<String>,
    pub lang: Option<String>,
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub sha256_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SourceTemplateContentResult {
    pub id: SourceId,
    pub source_template_id: SourceTemplateId,
    pub file_path: String,
    pub content: String,
    pub created_at: Option<i32>,
    pub updated_at: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceTemplateContentReadmeBoilermaker {
    pub readme: Option<SourceTemplateContentResult>,
    pub boilermaker: Option<SourceTemplateContentResult>,
}

impl From<Vec<SourceTemplateContentResult>> for SourceTemplateContentReadmeBoilermaker {
    fn from(results: Vec<SourceTemplateContentResult>) -> Self {
        let mut readme: Option<SourceTemplateContentResult> = None;
        let mut boilermaker: Option<SourceTemplateContentResult> = None;

        for r in results.into_iter() {
            if r.file_path.to_lowercase().contains("readme.") {
                readme = Some(r);
            } else if r.file_path.to_lowercase().starts_with("/boilermaker.") {
                boilermaker = Some(r);
            }
        }

        SourceTemplateContentReadmeBoilermaker {
            readme,
            boilermaker,
        }
    }
}
