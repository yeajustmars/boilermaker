use dioxus::prelude::*;

use crate::constants::{DROPDOWN_LINK_STYLE, NAVBAR_STYLE, WEB_FAVICON};
use crate::{MainSettings, WebSearch};

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            class: NAVBAR_STYLE,
            div { class: "w-1/4 text-2xl",
                MainHamburgerMenu {}
                MainNavDropdownMenu {}
                MainLink {}
            }
            div { class: "w-1/2", WebSearch {} }
            div { class: "w-1/4 text-right", MainSettings {} }
        }
    }
}

#[component]
fn MainHamburgerMenu() -> Element {
    rsx! {
        span {
            class: "pr-2",
            i { class: "fa-solid fa-bars" }
        }
    }
}

#[component]
fn MainLink() -> Element {
    rsx! {
        span { class: "ml-2 text-lg",
            a {
                href: "/",
                "Boilermaker"
                img { class: "inline h-6 w-6 mr-1", src: WEB_FAVICON }
            }
        }
    }
}

#[component]
fn MainNavDropdownMenu() -> Element {
    rsx! {
        div { class: "absolute left-0 top-13 w-48 bg-white dark:bg-neutral-900 rounded shadow-lg border border-l-0 border-t-0 border-neutral-300 dark:border-neutral-700 z-10 text-sm ",
            a { class: DROPDOWN_LINK_STYLE, href: "/" ,
                i { class: "fa-solid fa-house" }
                span { class: "ml-2", "Home" }
            }
            a { class: DROPDOWN_LINK_STYLE, href:  "/templates" ,
                i { class: "fa-solid fa-note-sticky" }
                span { class: "ml-2", "Templates" }
            }
            a { class: DROPDOWN_LINK_STYLE, href: "/docs" ,
                i { class: "fa-solid fa-file-code" }
                span { class: "ml-2", "Docs" }
            }
            a { class: DROPDOWN_LINK_STYLE, href: "/get-involved",
                i { class: "fa-solid fa-hands-helping" }
                span { class: "ml-2", "Get Involved" }
            }
        }
    }
}
