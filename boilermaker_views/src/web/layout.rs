use dioxus::prelude::*; // Import the api module from your crate (adjust the path if needed)

use crate::{
    FONT_AWESOME_URL, FONT_FIRA_CODE_URL, FONT_ROBOTO_URL, LAYOUT_STYLE, WEB_FAVICON, WEB_MAIN_CSS,
    WEB_TAILWIND_CSS, web::Navbar,
};

#[component]
pub fn HtmlLayout(children: Element) -> Element {
    rsx! {
        head {
            title { "Boilermaker - Project Templates Made Easy" }
            document::Link { rel: "icon", href: WEB_FAVICON }
            link { rel: "preconnect", href: "https://fonts.googleapis.com" }
            link {
                rel: "preconnect",
                href: "https://fonts.gstatic.com",
                crossorigin: true,
            }
            link { rel: "stylesheet", href: FONT_ROBOTO_URL }
            link { rel: "stylesheet", href: FONT_FIRA_CODE_URL}
            link { rel: "stylesheet", href: FONT_AWESOME_URL }
            link { rel: "stylesheet", href: WEB_MAIN_CSS }
            link { rel: "stylesheet", href: WEB_TAILWIND_CSS}
        }
        body {
            div { class: LAYOUT_STYLE,
                Navbar {}
                {children}
            }
        }
    }
}
