use color_eyre::{Result, eyre::eyre};
use lazy_static::lazy_static;
use regex::Regex;
use rust_embed::RustEmbed;
use sqlx::QueryBuilder;

use super::LocalCache;

#[derive(RustEmbed, Clone, Debug)]
#[folder = "../../packages/boilermaker_ui/docs/"]
struct DocFiles;

#[async_trait::async_trait]
pub trait DocMethods: Send + Sync {
    async fn index_docs(&self) -> Result<()>;
}

#[derive(Debug, Clone)]
struct Doc {
    content: String,
    created_at: i32,
    rel_path: String,
    title: Option<String>,
}

lazy_static! {
    static ref TITLE_REGEX: Regex = Regex::new(r"(?m)^#\s.*").unwrap();
}

#[async_trait::async_trait]
impl DocMethods for LocalCache {
    #[tracing::instrument]
    async fn index_docs(&self) -> Result<()> {
        let doc_rows = DocFiles::iter()
            .map(|file| {
                let f = DocFiles::get(&file).unwrap();
                let rel_path = file.as_ref().to_string();
                let created_at = f.metadata.created().unwrap();
                let content = str::from_utf8(f.data.as_ref()).unwrap().to_string();
                let title = TITLE_REGEX
                    .find(&content)
                    .map(|m| Some(m.as_str().trim_start_matches("# ").to_string()))
                    .unwrap_or(None);

                Doc {
                    content,
                    created_at: created_at as i32,
                    rel_path,
                    title,
                }
            })
            .collect::<Vec<Doc>>();

        let mut qb = QueryBuilder::new("INSERT INTO doc (rel_path, content, title, created_at) ");
        qb.push_values(doc_rows.iter(), |mut b, doc| {
            b.push_bind(&doc.rel_path)
                .push_bind(&doc.content)
                .push_bind(&doc.title)
                .push_bind(doc.created_at);
        });

        qb.build()
            .execute(&self.pool)
            .await
            .map_err(|e| eyre!("Failed to index docs: {e}"))?;

        Ok(())
    }
}
