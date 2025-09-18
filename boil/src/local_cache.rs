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
}

#[derive(Debug)]
pub struct TemplateRow {
    pub name: String,
    pub lang: String,
    pub template_dir: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
}

impl LocalCache {
    #[tracing::instrument]
    pub async fn new(path: &str) -> Result<LocalCache> {
        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        Ok(LocalCache { pool })
    }

    #[tracing::instrument]
    pub async fn template_table_exists(&self) -> Result<bool> {
        // TODO: rewrite with compile-time macros in sqlx
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
        // TODO: rewrite with compile-time macros in sqlx
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS template (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                lang TEXT,
                template_dir TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
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
    pub async fn add_template(&self, template: TemplateRow) -> Result<i64> {
        // TODO: rewrite with compile-time macros in sqlx
        let result = sqlx::query(
            r#"
            INSERT INTO template (name, lang, template_dir, repo, branch, subdir)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6);
            "#,
        )
        .bind(template.name)
        .bind(template.lang)
        .bind(template.template_dir)
        .bind(template.repo)
        .bind(template.branch)
        .bind(template.subdir)
        .execute(&self.pool)
        .await;

        match result {
            Ok(result) => Ok(result.last_insert_rowid()),
            Err(e) => {
                if e.to_string().contains("code: 2067") {
                    return Err(eyre!(
                        [
                            "💥 Template already exists in local cache.",
                            "(There is a unique connstraint for: name, repo, branch, subdir)",
                        ]
                        .join(" ")
                    ));
                } else {
                    return Err(eyre!("💥 Failed to add template to local cache: {}", e));
                }
            }
        }
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
