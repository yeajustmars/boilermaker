use dioxus::prelude::*;

#[component]
pub fn Docs() -> Element {
    rsx! {
        document::Title { "Boilermaker - Docs" }
        div { class: "py-4 px-2",
            h1 { "Docs" }
        }
    }
}
