use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use minijinja::context;

use crate::WebAppState;

pub async fn help(State(app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let name = "help.jinja";
    let context = context! {
        title => "Help",
        welcome_text => "Hello Help!",
    };
    Ok(Html(app.render(name, context)))
}
