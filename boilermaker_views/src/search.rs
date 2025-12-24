use dioxus::prelude::*;

use crate::constants::SEARCH_INPUT_STYLE;

#[component]
pub fn DesktopSearch() -> Element {
    rsx! {
        input {
            id: "search",
            class: SEARCH_INPUT_STYLE,
            placeholder: "Search...",
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
