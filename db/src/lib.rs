use color_eyre::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use tracing;

pub struct Database {
    pub pool: SqlitePool,
    pub path: String,
}

impl Database {
    #[tracing::instrument]
    pub async fn new(path: &str) -> Result<Database> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        Ok(Database {
            pool,
            path: path.to_owned(),
        })
    }
}
