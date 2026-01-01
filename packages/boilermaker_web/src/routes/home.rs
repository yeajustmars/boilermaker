use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use axum_template::TemplateEngine;
use minijinja::context;

use crate::{make_context, WebAppState};

pub async fn home(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let ctx = make_context(app.clone(), context! { title => "Home", });
    let out = app
        .template
        .render("home.html", ctx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(out))
}
