use std::fs::OpenOptions;

use color_eyre::{Result, eyre::eyre};
use lazy_static::lazy_static;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use tracing::info;

// TODO: move to a constants mod
lazy_static! {
    pub static ref BOILERMAKER_LOCAL_CACHE_PATH: PathBuf =
        make_boilermaker_local_cache_path().unwrap();
}

#[derive(Debug)]
pub struct LocalCache {
    pub pool: SqlitePool,
    pub path: String,
}

impl LocalCache {
    #[tracing::instrument]
    pub async fn new(path: &str) -> Result<LocalCache> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        Ok(LocalCache {
            pool,
            path: path.to_owned(),
        })
    }

    #[tracing::instrument]
    pub async fn initialize(&self) -> Result<()> {
        // TODO: move to migration
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS template (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                template TEXT NOT NULL,
                lang TEXT,
                branch TEXT,
                subdir TEXT,
                output TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[tracing::instrument]
pub fn make_boilermaker_local_cache_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Can't find home directory"))?;
    let local_cache_path = home_dir.join(".boilermaker").join("local_cache.db");

    if !local_cache_path.exists() {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&local_cache_path)
        {
            Ok(_) => (),
            Err(e) => return Err(eyre!("💥 Failed to create local cache file: {}", e)),
        };
    }

    info!(
        "Created boilermaker local cache directory: {}",
        local_cache_path.display()
    );

    Ok(local_cache_path)
}
