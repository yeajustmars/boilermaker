use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;

use boilermaker_views::{util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

#[component]
pub fn GetInvolvedPage() -> Element {
    rsx! {
        div {
            h1 { "Get Involved!" }
        }
    }
}

pub async fn get_involved(
    State(_app): State<Arc<WebAppState>>,
) -> Result<Html<String>, StatusCode> {
    let page = || {
        rsx! {
            HtmlLayout {
                GetInvolvedPage {}
            }
        }
    };
    Ok(Html(dioxus_to_html_page(page)))
}
