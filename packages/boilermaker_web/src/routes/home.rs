use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use axum_template::TemplateEngine;
use dioxus::asset_resolver::read_asset_bytes;
use minijinja::context;

use crate::WebAppState;
use boilermaker_ui::MAIN_CSS;

pub async fn home(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let main_css = String::from_utf8(read_asset_bytes(&MAIN_CSS).await.unwrap()).unwrap();
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    println!("MAIN_CSS: {MAIN_CSS:?}");
    println!("main_css: {main_css:?}");
    println!("%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%");
    let page = app
        .template
        .render("home.html", context! {})
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(page))
}
