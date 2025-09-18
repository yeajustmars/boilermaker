use std::fs::OpenOptions;

use color_eyre::{Result, eyre::eyre};
use lazy_static::lazy_static;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use tracing::info;

use crate::template::TemplateCommand;

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
    pub async fn template_table_exists(&self) -> Result<bool> {
        let row: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(name) FROM sqlite_master WHERE type='table' AND name='template';
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(row.0 > 0)
    }

    #[tracing::instrument]
    pub async fn create_template_table(&self) -> Result<()> {
        // TODO: move to migration
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS template (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                lang TEXT,
                branch TEXT,
                subdir TEXT,
                output_dir TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    #[tracing::instrument]
    pub async fn add_template(&self, template: TemplateCommand) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO template (name, lang, branch, subdir, output_dir)
            VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(template.name)
        .bind(template.lang)
        .bind(template.branch)
        .bind(template.subdir)
        .bind(template.output)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }
}

#[tracing::instrument]
pub fn make_boilermaker_local_cache_path() -> Result<PathBuf> {
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Can't find home directory"))?;
    let local_cache_path = home_dir.join(".boilermaker").join("local_cache.db");

    match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&local_cache_path)
    {
        Ok(_) => {
            info!(
                "Created boilermaker local cache file: {}",
                local_cache_path.display()
            );
            Ok(local_cache_path)
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                return Ok(local_cache_path);
            } else {
                return Err(eyre!("💥 Failed to create local cache file: {}", e));
            }
        }
    }
}
