use color_eyre::{Result, eyre::eyre};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use sqlx::QueryBuilder;
use tabled::Tabled;

use super::LocalCache;
use crate::{db::SearchOptions, docs::DocFiles};

#[async_trait::async_trait]
pub trait DocMethods: Send + Sync {
    async fn index_docs(&self, opts: Option<IndexDocsOptions>) -> Result<()>;
    async fn get_doc(&self, id: DocumentId) -> Result<DocRow>;
    async fn get_docs(&self) -> Result<Vec<DocRow>>;
    async fn find_docs(&self, query: DocFindParams) -> Result<Vec<DocRow>>;
    async fn search_docs(&self, term: &str, opts: Option<SearchOptions>) -> Result<Vec<DocResult>>;
}

#[async_trait::async_trait]
impl DocMethods for LocalCache {
    #[tracing::instrument]
    async fn index_docs(&self, _opts: Option<IndexDocsOptions>) -> Result<()> {
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

    #[tracing::instrument]
    async fn get_doc(&self, id: DocumentId) -> Result<DocRow> {
        let row = sqlx::query_as::<_, DocRow>("SELECT * FROM doc WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| eyre!("Failed to get doc with id {id}: {e}"))?;

        Ok(row)
    }

    #[tracing::instrument]
    async fn get_docs(&self) -> Result<Vec<DocRow>> {
        let rows = sqlx::query_as::<_, DocRow>(
            r#"
            SELECT id, rel_path, content, title, created_at
            FROM doc
            ORDER BY rel_path ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| eyre!("Failed to get docs: {e}"))?;

        Ok(rows)
    }

    #[tracing::instrument]
    async fn find_docs(&self, query: DocFindParams) -> Result<Vec<DocRow>> {
        let mut qb = QueryBuilder::new("SELECT * FROM doc WHERE 1=1 ");

        if let Some(rel_path) = query.rel_path {
            qb.push("AND rel_path = ").push_bind(rel_path);
        }
        if let Some(content) = query.content {
            qb.push("AND content LIKE ")
                .push_bind(format!("%{content}%"));
        }
        if let Some(title) = query.title {
            qb.push("AND title LIKE ").push_bind(format!("%{title}%"));
        }
        // TODO: make date find more flexible (e.g. date range)
        if let Some(created_at) = query.created_at {
            qb.push("AND created_at = ").push_bind(created_at);
        }

        let q = qb.build_query_as::<DocRow>();
        let results = q.fetch_all(&self.pool).await?;

        Ok(results)
    }

    #[tracing::instrument]
    async fn search_docs(
        &self,
        term: &str,
        _opts: Option<SearchOptions>,
    ) -> Result<Vec<DocResult>> {
        let results = sqlx::query_as::<_, DocResult>(
            r#"
            SELECT doc.*
            FROM doc_fts
                LEFT JOIN doc AS doc ON doc_fts.rowid = doc.id
            WHERE
                doc_fts MATCH ?
            "#,
        )
        .bind(term)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| eyre!("Failed to search docs with term '{term}': {e}"))?;

        Ok(results)
    }
}

lazy_static! {
    static ref TITLE_REGEX: Regex = Regex::new(r"(?m)^#\s.*").unwrap();
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub content: String,
    pub created_at: i32,
    pub rel_path: String,
    pub title: Option<String>,
}

pub type DocumentId = i64;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct DocRow {
    pub id: DocumentId,
    pub content: String,
    pub created_at: i32,
    pub rel_path: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IndexDocsOptions {
    pub dev: bool,
}

#[derive(Debug, Tabled)]
pub struct TabledDocRow {
    pub id: DocumentId,
    pub content: String,
    pub created_at: i32,
    pub rel_path: String,
    pub title: String,
}

impl From<DocRow> for TabledDocRow {
    fn from(doc: DocRow) -> Self {
        Self {
            id: doc.id,
            content: doc.content,
            created_at: doc.created_at,
            rel_path: doc.rel_path,
            title: doc.title.unwrap_or("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocFindParams {
    pub id: Option<DocumentId>,
    pub content: Option<String>,
    pub created_at: Option<i32>,
    pub rel_path: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct DocResult {
    pub id: DocumentId,
    pub content: String,
    pub created_at: i32,
    pub rel_path: String,
    pub title: Option<String>,
}
