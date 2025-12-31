use dioxus::prelude::*;

use crate::{DROPDOWN_MENU_STYLE, FAVICON, NAVBAR_STYLE};

static MAIN_DROPDOWN_OPEN_STATE: GlobalSignal<bool> = Signal::global(|| false);

fn close_main_dropdown() {
    *MAIN_DROPDOWN_OPEN_STATE.write() = false;
}

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        div {
            id: "navbar",
            class: NAVBAR_STYLE,
            onmouseleave: move |_| close_main_dropdown(),

            div { class: "w-1/4 text-2xl",
                MainNavDropdownMenu {
                    {children}
                }
                MainLinks {}
            }
            //div { class: "w-1/2", DesktopSearch {} }
            //div { class: "w-1/4 text-right", MainSettings {} }
        }
    }
}

#[component]
fn MainLinks() -> Element {
    rsx! {
        span { class: "ml-2 text-lg",
            Link { to: "/", "Boilermaker" }
            img { class: "inline h-6 w-6 mr-1",
                src: FAVICON }
        }
    }
}

#[component]
fn MainNavDropdownMenu(children: Element) -> Element {
    let is_open = *MAIN_DROPDOWN_OPEN_STATE.read();

    rsx! {
        span {
            class: "pr-2",
            onclick: move |_| {
                *MAIN_DROPDOWN_OPEN_STATE.write() = !is_open;
            },
            i { class: "fa-solid fa-bars" }
        }
        if is_open {
            div {
                onclick: move |_| close_main_dropdown(),
                onmouseleave: move |_| close_main_dropdown(),
                class: DROPDOWN_MENU_STYLE,
                {children}
            }
        }
    }
}
