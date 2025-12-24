use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;

use boilermaker_views::{util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            class: "w-full p-8",
            h1 { "Welcome to Boilermaker!" }
            div {
                class: "grid grid-cols-1 md:grid-cols-3 gap-4",
                div {
                    class: "bg-blue-200 p-4",
                    h3 { "Column 1" }
                    p { "Content for the first column." }
                }
                div {
                    class: "bg-green-200 p-4",
                    h3 { "Column 2" }
                    p { "Content for the second column." }
                }
                div {
                    class: "bg-red-200 p-4",
                    h3 { "Column 3" }
                    p { "Content for the third column." }
                }
            }
        }
    }
}

pub async fn home(State(_app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let page = || {
        rsx! {
            HtmlLayout {
                HomePage {}
            }
        }
    };
    Ok(Html(dioxus_to_html_page(page)))
}
