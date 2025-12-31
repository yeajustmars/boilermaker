use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use axum_template::TemplateEngine;
use minijinja::context;

use crate::WebAppState;

pub async fn home(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let page = app
        .template
        .render("home.html", context! {})
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(page))
}
