use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;

use boilermaker_views::{util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

#[component]
pub fn SettingsPage() -> Element {
    rsx! {
        div {
            h1 { "Settings" }
        }
    }
}

pub async fn settings(State(_app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let page = || {
        rsx! {
            HtmlLayout {
                SettingsPage {}
            }
        }
    };
    Ok(Html(dioxus_to_html_page(page)))
}
