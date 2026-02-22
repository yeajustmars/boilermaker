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
use boilermaker_core::{
    db::{DocResult, SearchOptions, SearchResult, SearchResultKind},
    docs::rel_url,
};

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

// TODO: implement source search
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SearchParamsCategory {
    All,
    Doc,
    //Source,
    Template,
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
        match params.category.unwrap_or(SearchParamsCategory::All) {
            SearchParamsCategory::All => {
                let template_results = search_templates(&app, &term, opts.clone())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let doc_results = search_docs(&app, &term, opts.clone())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let all_results: Vec<WebSearchResult> = template_results
                    .into_iter()
                    .map(WebSearchResult::Template)
                    .chain(doc_results.into_iter().map(WebSearchResult::Doc))
                    // TODO: sort results by common field
                    .collect();
                println!("Search results:\n{:#?}", all_results);

                context! { search_results => all_results }
            }
            SearchParamsCategory::Doc => {
                let doc_results = search_docs(&app, &term, opts.clone())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let all_results: Vec<WebSearchResult> =
                    doc_results.into_iter().map(WebSearchResult::Doc).collect();

                context! { search_results => all_results }
            }
            // TODO: implement source search
            //SearchParamsCategory::Source => context! {},
            SearchParamsCategory::Template => {
                let template_results = search_templates(&app, &term, opts.clone())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let all_results: Vec<WebSearchResult> = template_results
                    .into_iter()
                    .map(WebSearchResult::Template)
                    .collect();

                context! { search_results => all_results }
            }
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

async fn search_templates(
    app: &Arc<WebAppState>,
    term: &str,
    opts: SearchOptions,
) -> Result<Vec<WebSearchTemplateResult>> {
    let db = app.db.clone();
    let results = db
        .search_sources(None, term, Some(opts))
        .await?
        .into_iter()
        .map(|res: SearchResult| WebSearchTemplateResult::to_context(res, term))
        .collect::<Vec<WebSearchTemplateResult>>();

    Ok(results)
}

async fn search_docs(
    app: &Arc<WebAppState>,
    term: &str,
    opts: SearchOptions,
) -> Result<Vec<WebSearchDocResult>> {
    let db = app.db.clone();
    let results = db
        .search_docs(term, Some(opts))
        .await?
        .into_iter()
        .map(|res: DocResult| WebSearchDocResult::to_context(res, term))
        .collect::<Vec<WebSearchDocResult>>();
    Ok(results)
}

// async fn search_sources(
//     _app: &Arc<WebAppState>,
//     _term: &str,
//     _opts: SearchOptions,
// ) -> Result<Vec<WebSearchTemplateResult>> {
//     Ok(vec![])
// }

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
pub enum WebSearchResult {
    Template(WebSearchTemplateResult),
    Doc(WebSearchDocResult),
    Source(WebSearchTemplateResult),
}

#[derive(Debug, Clone, Serialize)]
pub struct WebSearchTemplateResult {
    pub kind: String,
    pub id: i64,
    pub name: String,
    pub lang: String,
    pub repo: String,
    pub branch: Option<String>,
    pub subdir: Option<String>,
    pub highlighted_lines: Option<Vec<HighlightedLine>>,
}

impl WebSearchTemplateResult {
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

#[derive(Debug, Clone, Serialize)]
pub struct WebSearchDocResult {
    pub kind: String,
    pub id: i64,
    pub title: Option<String>,
    pub rel_path: String,
    pub rel_url: String,
    pub highlighted_lines: Option<Vec<HighlightedLine>>,
}

impl WebSearchDocResult {
    pub fn to_context(res: DocResult, term: &str) -> Self {
        let rel_url = rel_url(&res.rel_path);
        let highlighted_lines = highlight_search_term(&res.content, term);

        Self {
            kind: "doc".to_string(),
            id: res.id,
            title: res.title,
            rel_path: res.rel_path,
            rel_url,
            highlighted_lines: Some(highlighted_lines),
        }
    }
}
