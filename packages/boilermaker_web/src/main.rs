// web::main

use dioxus::prelude::*;

use boilermaker_ui::{Navbar, DROPDOWN_LINK_STYLE, INDENTED_DROPDOWN_LINK_STYLE};
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

pub const FONT_AWESOME_URL: &str =
    "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css";
pub const FONT_ROBOTO_URL: &str =
    "https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap";
pub const FONT_FIRA_CODE_URL: &str =
    "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;600;700&display=swap";

pub const LAYOUT_STYLE: &str =
    "min-h-screen bg-white text-neutral-800 dark:bg-neutral-900 dark:text-neutral-200";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Stylesheet { href: FONT_AWESOME_URL }
        document::Stylesheet { href: FONT_ROBOTO_URL }
        document::Stylesheet { href: FONT_FIRA_CODE_URL}

        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        div {
            id: "layout",
            class: LAYOUT_STYLE,
            WebNavbar {}

            Outlet::<Route> {}
        }
    }
}
/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {
        Navbar {
            /*
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
             */
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Home {},
                    i { class: "fa-solid fa-house" }
                    span { class: "ml-2", "Home" }
                }
                /*
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Templates {},
                    i { class: "fa-solid fa-note-sticky" }
                    span { class: "ml-2", "Templates" }
                }
                Link {
                    class: INDENTED_DROPDOWN_LINK_STYLE,
                    to: Route::TemplateAdd {},
                    i { class: "fa-solid fa-plus" }
                    span { class: "ml-2", "Add Template" }
                }
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Docs {},
                    i { class: "fa-solid fa-file-code" }
                    span { class: "ml-2", "Docs" }
                }
                Link { class: DROPDOWN_LINK_STYLE, to: Route::GetInvolved {},
                    i { class: "fa-solid fa-hands-helping" }
                    span { class: "ml-2", "Get Involved" }
                }
                 */
        }

        Outlet::<Route> {}
    }
}
