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
use boilermaker_core::db::{SearchResult, SearchResultKind};

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
    //println!("Search params: {:?}", params);
    //let ten_seconds = std::time::Duration::from_secs(1);
    //std::thread::sleep(ten_seconds);

    let term = params.term;
    let search_results: JinjaValue = {
        let db = app.db.clone();
        match params.category.unwrap_or(SearchParamsCategory::All) {
            SearchParamsCategory::All => {
                let template_results = db
                    .search_sources(None, &term)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
                context! { search_results => template_results }
                // TODO: add doc_results
            }
            SearchParamsCategory::Template => context! {},
            SearchParamsCategory::Doc => context! {},
        }
    };

    let base_ctx = make_context(context! { title => "Search Results" });
    let ctx = merge_maps([base_ctx, search_results]);

    let out = app
        .template
        .render("search_results.html", ctx)
        .map_err(|e| {
            error!("Search error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Html(out))
}

fn search_templates() -> Vec<JinjaValue> {
    vec![]
}

fn search_docs() -> Vec<JinjaValue> {
    vec![]
}

/*
pub async fn search_templates(
    cache: Arc<dyn TemplateDb>,
    term: &str,
    scope: SearchScope,
) -> Result<Vec<SearchResult>> {
    match scope {
        SearchScope::Local => Ok(cache.search_templates(term).await?),
        SearchScope::Source(name) => Ok(cache.search_sources(Some(name), term).await?),
        SearchScope::All => {
            let mut all_results = Vec::new();
            let local = cache.search_templates(term).await?;
            all_results.extend(local);

            let sources = cache.search_sources(None, term).await?;
            all_results.extend(sources);
            Ok(all_results)
        }
    }
}
 */
