use std::str;

use color_eyre::Result;
use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool},
};

use super::{DocMethods, SourceMethods, TemplateMethods};

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

//pub const DOCS_DIR: &str = "../../packages/boilermaker_ui/docs/";

#[async_trait::async_trait]
pub trait TemplateDb: TemplateMethods + SourceMethods + DocMethods + Send + Sync {
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
