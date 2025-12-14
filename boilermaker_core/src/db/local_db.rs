use color_eyre::Result;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool},
};

pub use super::{
    template::{
        hash_template_row, DisplayableTemplateListResult, ListTemplateOptions, SearchResult,
        TemplateFindParams, TemplateMethods, TemplateResult, TemplateRow,
    },
    SourceMethods,
};

static MIGRATOR: Migrator = sqlx::migrate!("../migrations");

#[async_trait::async_trait]
pub trait TemplateDb: TemplateMethods + SourceMethods + Send + Sync {
    // TODO: rename create_schema or similar (now local_db has cache + sources)
    async fn create_schema(&self) -> Result<()>;
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
    async fn create_schema(&self) -> Result<()> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }
}
