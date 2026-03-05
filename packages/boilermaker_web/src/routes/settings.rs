use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use axum_template::TemplateEngine;
use color_eyre::eyre::Result;
use minijinja::context;

use crate::{WebAppState, make_context};

#[tracing::instrument]
pub async fn settings(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let ctx = make_context(context! { title => "Settings", });
    let out = app
        .template
        .render("settings.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(out))
}
