use dioxus::prelude::*;

use views::Echo;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Boilermaker" }
        //TODO: decide on switching between light/dark modes for code blocks
        // document::Stylesheet { href: GITHUB_LIGHT_CSS }
        div { class: "py-4 px-2",
            h1 { class: "text-3xl font-bold", "Boilermaker Desktop App" }
        }
        Echo {}
    }
}
