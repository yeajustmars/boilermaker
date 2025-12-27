use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;
use dioxus::ssr::render_element;

use boilermaker_views::{util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

pub async fn docs(State(_app_state): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    /*
    let page = || {
        rsx! {
            HtmlLayout {
                //DocsPage {}
            }
        }
    };
    Ok(Html(dioxus_to_html_page(page)))
     */
    Ok(Html(render_element(rsx! {
        HtmlLayout {
            DocsPage {}
        }
    })))
}

#[component]
pub fn DocsPage() -> Element {
    rsx! {
        div {
            h1 { "Docs" }
        }
    }
}
