use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};

use crate::WebAppState;

pub async fn get_involved(
    State(_app): State<Arc<WebAppState>>,
) -> Result<Html<String>, StatusCode> {
    Ok(Html("<h1>Get Involved</h1>".to_string()))
}
