use dioxus::prelude::*;

#[component]
pub fn NewProject(i: usize) -> Element {
    rsx! { "PreRender template at index #{i}" }
}
