use color_eyre::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

#[async_trait::async_trait]
pub trait TemplateDb: Send + Sync {
    async fn create_template_table(&self) -> Result<()>;
    async fn template_table_exists(&self) -> Result<bool>;
    async fn create_template(&self) -> Result<i64>;
    async fn update_template(&self) -> Result<TemplateResult>;
    async fn delete_template(&self) -> Result<i64>;
    async fn get_template(&self) -> Result<TemplateResult>;
    async fn list_templates(&self) -> Vec<TemplateResult>;
    async fn search_templates(&self) -> Vec<TemplateResult>;
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
    // TODO: move to migration
    // TODO: rewrite with compile-time macros in sqlx
    async fn create_template_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS template (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                lang TEXT,
                template_dir TEXT,
                created_at TIMESTAMP NOT NULL, 
                updated_at TIMESTAMP, 
                repo TEXT,
                branch TEXT,
                subdir TEXT,
                UNIQUE (name, repo, branch, subdir)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
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
    async fn create_template(&self) -> Result<i64> {
        todo!()
    }

    #[tracing::instrument]
    async fn update_template(&self) -> Result<TemplateResult> {
        todo!()
    }

    #[tracing::instrument]
    async fn delete_template(&self) -> Result<i64> {
        todo!()
    }

    #[tracing::instrument]
    async fn get_template(&self) -> Result<TemplateResult> {
        todo!()
    }

    #[tracing::instrument]
    async fn list_templates(&self) -> Vec<TemplateResult> {
        todo!()
    }

    #[tracing::instrument]
    async fn search_templates(&self) -> Vec<TemplateResult> {
        todo!()
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TemplateResult {
    pub id: i64,
    pub name: String,
    pub template: String,
    pub lang: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub overwrite: bool,
}
