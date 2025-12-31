use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/css/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/css/tailwind.css");
const FONT_AWESOME_URL: &str =
    "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css";
const FONT_ROBOTO_URL: &str =
    "https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap";
const FONT_FIRA_CODE_URL: &str =
    "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;600;700&display=swap";

#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        document::Title { "Boilermaker - Project Templates Made Easy" }
        link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: true, }
        document::Stylesheet { href: FONT_AWESOME_URL }
        document::Stylesheet { href: FONT_ROBOTO_URL }
        document::Stylesheet { href: FONT_FIRA_CODE_URL}
        document::Stylesheet { href: TAILWIND_CSS }
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "layout",
            class: "min-h-screen bg-white text-neutral-800 dark:bg-neutral-900 dark:text-neutral-200",
            {children}
        }
    }
}
