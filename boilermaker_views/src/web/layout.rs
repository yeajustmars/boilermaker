use dioxus::prelude::*; // Import the api module from your crate (adjust the path if needed)

use crate::{
    FAVICON, FONT_AWESOME_URL, FONT_FIRA_CODE_URL, FONT_ROBOTO_URL, MAIN_CSS, TAILWIND_CSS,
};

#[component]
pub fn HtmlLayout(children: Element) -> Element {
    rsx! {
        head {
            title { "Boilermaker - Project Templates Made Easy" }
            link { rel: "icon", href: FAVICON }
            link { rel: "preconnect", href: "https://fonts.googleapis.com" }
            link {
                rel: "preconnect",
                href: "https://fonts.gstatic.com",
                crossorigin: true,
            }
            link { rel: "stylesheet", href: FONT_AWESOME_URL }
            link { rel: "stylesheet", href: FONT_ROBOTO_URL }
            link { rel: "stylesheet", href: FONT_FIRA_CODE_URL}
            link { rel: "stylesheet", href: MAIN_CSS}
            link { rel: "stylesheet", href: TAILWIND_CSS}
        }
        body {
            {children}
        }
    }
}
