use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
};
use axum_template::TemplateEngine;
use color_eyre::eyre::Result;
use minijinja::context;
use pulldown_cmark::{html, Options, Parser};

use crate::{make_context, WebAppState};
use boilermaker_core::docs::{build_docs_tree, DocFiles};

#[tracing::instrument]
pub async fn docs(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let docs = app
        .db
        .get_docs()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let docs_tree = build_docs_tree(docs);

    let ctx = make_context(context! {
        title => "Docs",
        docs_tree,
    });
    let out = app
        .template
        .render("docs.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Html(out))
}

// TODO: fix single doc request with HTMX, to not do full rerender of page (sidebar, etc).
// NOTE: to get this out the door, whack-it-with-a-hammer ;)
#[tracing::instrument]
pub async fn doc(
    State(app): State<Arc<WebAppState>>,
    Path(path): Path<String>,
) -> Result<Html<String>, StatusCode> {
    // TODO: set title to first `# ...` in markdown
    let title = "Docs";

    let template = format!("{}.md", path);
    let content = match DocFiles::get(&template) {
        Some(file) => {
            let bytes = file.data.to_vec();
            String::from_utf8(bytes).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
        None => "File not found: {}".to_string(),
    };
    let options = Options::empty();
    let parser = Parser::new_ext(&content, options);
    let doc_page = {
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    };

    // TODO: replace this w/ HTMX; there's no need to rebuild this _every_ request.
    let docs = app
        .db
        .get_docs()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let docs_tree = build_docs_tree(docs);

    let ctx = make_context(context! {
        title => title,
        doc_page => doc_page,
        docs_tree,
    });

    let out = app
        .template
        .render("doc.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(out))
}

// TODO: move docs sidebar to generic location, if it makes sense.
