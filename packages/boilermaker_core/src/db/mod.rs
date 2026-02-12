use serde::Serialize;

pub mod doc;
pub mod local_db;
pub mod source;
pub mod template;

pub use doc::*;
pub use local_db::*;
pub use source::*;
pub use template::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum SearchResultKind {
    Template,
    Source,
}

impl std::fmt::Display for SearchResultKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchResultKind::Template => write!(f, "template"),
            SearchResultKind::Source => write!(f, "source"),
        }
    }
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub kind: SearchResultKind,
    pub id: i64,
    pub name: String,
    pub lang: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub content: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SearchOptions {
    pub content: bool,
}
