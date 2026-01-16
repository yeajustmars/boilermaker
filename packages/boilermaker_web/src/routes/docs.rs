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

use crate::{make_context, Docs, WebAppState};

#[tracing::instrument]
pub async fn docs(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let ctx = make_context(context! { title => "Docs", });
    let out = app
        .template
        .render("docs.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(out))
}

#[tracing::instrument]
pub async fn doc(
    State(app): State<Arc<WebAppState>>,
    Path(path): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let title = "Docs";

    let template = format!("{}.md", path);
    let content = match Docs::get(&template) {
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

    let ctx = make_context(context! {
        title => title,
        doc_page => doc_page,
    });

    let out = app
        .template
        .render("doc.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(out))
}
