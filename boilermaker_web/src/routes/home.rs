use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;

use boilermaker_views::{util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            h1 { "Welcome to Boilermaker!" }
            p { "Your one-stop solution for project templates." }
        }
    }
}

pub async fn home(State(_app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let page = || {
        rsx! {
            HtmlLayout {
                Home {}
            }
        }
    };
    Ok(Html(dioxus_to_html_page(page)))
}
