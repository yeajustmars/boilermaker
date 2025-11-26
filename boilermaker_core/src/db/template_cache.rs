use color_eyre::Result;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use tabled::Tabled;

use crate::util::crypto::sha256_hash_string;
use crate::util::time::timestamp_to_iso8601;

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[async_trait::async_trait]
pub trait TemplateDb: Send + Sync {
    async fn check_unique(&self, row: &TemplateRow) -> Result<Option<TemplateResult>>;
    async fn create_template(&self, row: TemplateRow) -> Result<i64>;
    async fn create_template_tables(&self) -> Result<()>;
    async fn delete_template(&self, id: i64) -> Result<i64>;
    async fn find_templates(&self, query: TemplateFindParams) -> Result<Vec<TemplateResult>>;
    async fn get_template(&self, id: i64) -> Result<Option<TemplateResult>>;
    async fn list_templates(
        &self,
        opts: Option<ListTemplateOptions>,
    ) -> Result<Vec<TemplateResult>>;
    async fn template_table_exists(&self) -> Result<bool>;
    async fn update_template(&self, id: i64, row: TemplateRow) -> Result<i64>;
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
        let result = sqlx::query(
            r#"
            INSERT INTO template (name, lang, template_dir, created_at, repo, branch, subdir, sha256_hash)
            VALUES (?, ?, ?, strftime('%s','now'), ?, ?, ?, ?);
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

        Ok(result.last_insert_rowid())
    }

    #[tracing::instrument]
    async fn create_template_tables(&self) -> Result<()> {
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
        let mut query = String::from("SELECT * FROM template WHERE 1=1"); //TODO: clean up the 1=1 hack
        let mut bindings: Vec<String> = Vec::new();

        if let Some(name) = params.name {
            query.push_str(" AND name = ?");
            bindings.push(name);
        }
        if let Some(lang) = params.lang {
            query.push_str(" AND lang = ?");
            bindings.push(lang);
        }
        if let Some(repo) = params.repo {
            query.push_str(" AND repo = ?");
            bindings.push(repo);
        }
        if let Some(branch) = params.branch {
            query.push_str(" AND branch = ?");
            bindings.push(branch);
        }
        if let Some(subdir) = params.subdir {
            query.push_str(" AND subdir = ?");
            bindings.push(subdir);
        }

        query.push_str(" ORDER BY name ASC");

        let mut q = sqlx::query_as::<_, TemplateResult>(&query);
        for binding in &bindings {
            q = q.bind(binding);
        }

        let results = q.fetch_all(&self.pool).await?;

        Ok(results)
    }

    // TODO: add options for ordering, pagination, filtering, etc
    #[tracing::instrument]
    async fn list_templates(
        &self,
        _opts: Option<ListTemplateOptions>,
    ) -> Result<Vec<TemplateResult>> {
        let results = sqlx::query_as::<_, TemplateResult>(
            r#"
            SELECT *
            FROM template
            ORDER BY created_at DESC;
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    #[tracing::instrument]
    async fn get_template(&self, id: i64) -> Result<Option<TemplateResult>> {
        let result = sqlx::query_as::<_, TemplateResult>(
            r#"
            SELECT *
            FROM template
            WHERE id = ?;
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
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

#[derive(Debug, Clone)]
pub struct TemplateFindParams {
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
