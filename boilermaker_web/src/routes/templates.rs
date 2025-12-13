use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;

use boilermaker_views::{util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

#[component]
pub fn TemplatesPage() -> Element {
    rsx! {
        div {
            h1 { "Templates" }
        }
    }
}

pub async fn templates(State(_app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let page = || {
        rsx! {
            HtmlLayout {
                TemplatesPage {}
            }
        }
    };
    Ok(Html(dioxus_to_html_page(page)))
}
