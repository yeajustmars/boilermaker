use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use minijinja::context;

use crate::WebAppState;

pub async fn home(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let name = "home.jinja";
    let context = context! {
        title => "Home",
        welcome_text => "Hello World!",
    };
    Ok(Html(app.render(name, context)))
}
