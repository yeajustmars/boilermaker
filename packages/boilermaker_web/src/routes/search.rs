use std::sync::Arc;

use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::Html,
};
use axum_template::TemplateEngine;
use color_eyre::Result;
use minijinja::{
    context,
    value::{merge_maps, Value as JinjaValue},
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{make_context, WebAppState};
use boilermaker_core::db::{SearchOptions, SearchResult, SearchResultKind};

/*
enum SearchError {
    InvalidQuery,
}

// TODO: impl custom error details w/o EVER revealing the underlying logged error
// TODO: make error.html pretty
fn search_error(app: Arc<WebAppState>, cause: SearchError) -> Result<Html<String>, StatusCode> {
    match cause {
        SearchError::InvalidQuery => {
            let ctx = make_context(context! {
                title => "Search Error",
                status => 400,
                error_msg => "400 Bad Request",
                error_details => "Invalid query.",
            });
            let err_page = app
                .template
                .render("search_error.html", ctx)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(Html(err_page))
        }
    }
}
 */

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SearchParamsCategory {
    All,
    Template,
    Doc,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchParams {
    pub term: String,
    pub lang: Option<String>,
    pub kind: Option<String>,
    pub category: Option<SearchParamsCategory>,
}

// TODO: add docs to DB for searching
// TODO: implement category search
pub async fn search(
    State(app): State<Arc<WebAppState>>,
    Form(params): Form<SearchParams>,
) -> Result<Html<String>, StatusCode> {
    let term = params.term;
    let opts = SearchOptions { content: true };

    let search_results: JinjaValue = {
        let db = app.db.clone();
        match params.category.unwrap_or(SearchParamsCategory::All) {
            SearchParamsCategory::All => {
                let template_results = db
                    .search_sources(None, &term, Some(opts))
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                    .into_iter()
                    .map(|res: SearchResult| WebSearchResult::to_context(res, &term))
                    .collect::<Vec<WebSearchResult>>();
                context! { search_results => template_results }
            }
            SearchParamsCategory::Template => context! {},
            SearchParamsCategory::Doc => context! {},
        }
    };

    let base_ctx = make_context(context! { title => "Search Results" });
    let ctx = merge_maps([base_ctx, search_results]);
    println!("ctx: {:#?}", ctx);

    let out = app
        .template
        .render("search_results.html", ctx)
        .map_err(|e| {
            error!("Search error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(out))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HighlightedLine {
    pub line_number: u16,
    pub line: String,
}

pub fn highlight_search_term(content: &str, term: &str) -> Vec<HighlightedLine> {
    let escaped_term = regex::escape(term);
    let re = regex::RegexBuilder::new(&escaped_term)
        .case_insensitive(true)
        .build()
        .unwrap();

    let mut relevant_lines: Vec<HighlightedLine> = Vec::new();
    for (index, line) in content.lines().enumerate() {
        if line.contains(term) {
            let highlighted = re
                .replace_all(line, |caps: &regex::Captures| {
                    format!(r#"<span class="highlighted">{}</span>"#, &caps[0])
                })
                .to_string();
            relevant_lines.push(HighlightedLine {
                line_number: (index + 1) as u16,
                line: highlighted,
            });
        }
    }
    relevant_lines
}

#[derive(Debug, Clone, Serialize)]
pub struct WebSearchResult {
    pub kind: String,
    pub id: i64,
    pub name: String,
    pub lang: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub highlighted_lines: Option<Vec<HighlightedLine>>,
}

impl WebSearchResult {
    pub fn to_context(res: SearchResult, term: &str) -> Self {
        let highlighted_lines = if (res.kind == SearchResultKind::Template
            || res.kind == SearchResultKind::Source)
            && res.content.is_some()
        {
            res.content.map(|c| highlight_search_term(&c, term))
        } else {
            None
        };

        Self {
            kind: res.kind.to_string(),
            id: res.id,
            name: res.name,
            lang: res.lang,
            repo: res.repo,
            branch: res.branch,
            subdir: res.subdir,
            highlighted_lines,
        }
    }
}
