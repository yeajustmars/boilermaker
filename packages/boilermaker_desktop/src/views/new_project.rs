use dioxus::prelude::*;

#[component]
pub fn NewProject(i: usize) -> Element {
    rsx! {
        h1 { "New Project {i}" }
    }
}
