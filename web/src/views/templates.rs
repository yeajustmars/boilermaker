use dioxus::prelude::*;

#[component]
pub fn Templates() -> Element {
    rsx! {
        document::Title { "Boilermaker - Templates" }
        div { class: "py-4 px-2",
            h1 { "Templates" }
        }
    }
}
