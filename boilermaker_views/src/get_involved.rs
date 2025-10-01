use dioxus::prelude::*;

#[component]
pub fn GetInvolved() -> Element {
    rsx! {
        document::Title { "Boilermaker - Get Involved" }
        div { class: "py-4 px-2",
            h1 { "Get Involved" }
        }
    }
}
