use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;

use boilermaker_views::{constants::LINK_STYLE, util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

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

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div {
            class: "w-full p-8",
            h1 { class: "text-2xl text-blue-400 mb-4 px-4",
                "Welcome to Boilermaker!"
            }
            div {
                class: "flex sm:flex-col md:flex-row gap-4",
                div {
                    class: "sm:w-full md:w-1/3 p-4",
                    h3 { class: "text-2xl",
                        "Latest Templates"
                    }
                    p { "TODO: list templates." }
                }
                div {
                    class: "sm:w-full md:w-1/3 p-4",
                    h3 { class: "text-2xl",
                        "Get Involved!"
                    }
                    p {
                        "The Boilermaker project is always looking for maintainers."
                        "If that's something you're interested in, please "
                        a { href: "/contact", class: LINK_STYLE, "contact us" }
                        "."
                    }
                }
                div {
                    class: "sm:w-full md:w-1/3 p-4",
                    h3 { class: "text-2xl",
                        "Column 3"
                    }
                    p { "Content for the third column." }
                }
            }
        }
    }
}
