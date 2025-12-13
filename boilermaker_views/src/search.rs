use dioxus::prelude::*;

use boilermaker_api;

use crate::constants::SEARCH_INPUT_STYLE;

#[component]
pub fn DesktopSearch() -> Element {
    let mut response = use_signal(String::new);

    rsx! {
        input {
            id: "search",
            class: SEARCH_INPUT_STYLE,
            placeholder: "Search...",
            oninput: move |event| async move {
                let data = boilermaker_api::search(event.value()).await.unwrap();
                response.set(data);
            },
        }
    }
}

#[component]
pub fn WebSearch() -> Element {
    rsx! {
        input {
            id: "search",
            class: SEARCH_INPUT_STYLE,
            placeholder: "Search...",
        }
    }
}
