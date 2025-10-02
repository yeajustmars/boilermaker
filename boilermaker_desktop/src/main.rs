use color_eyre::eyre::Result;
use dioxus::prelude::*;

mod ui;
use ui::{Home, TemplateNew};

use boilermaker_desktop::init_app_state;

use boilermaker_views::{
    Docs, GetInvolved, Search, Templates,
    {DROPDOWN_LINK_STYLE, FAVICON, INDENTED_DROPDOWN_LINK_STYLE, MAIN_CSS, TAILWIND_CSS},
};

//TODO: 1. [ ] Add a WASM-compiled playground for users to develop templates directly in the browser
//TODO: 2. [ ] Add a user login system to save favorite templates and settings

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/docs")]
        Docs {},
        #[route("/templates")]
        Templates {},
        #[route("/templates/new")]
        TemplateNew {},
        #[route("/get-involved")]
        GetInvolved {},
}

fn main() -> Result<()> {
    init_app_state()?;
    dioxus::launch(App);
    Ok(())
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Title { "Boilermaker - Project Templates Made Easy" }
        link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: true,
        }
        // TODO: move to Dioxus.toml
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;600;700&display=swap" }
        // TODO: move to Dioxus.toml
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn Layout() -> Element {
    rsx! {
        div {
            id: "layout",
            class: "min-h-screen bg-white text-neutral-800 dark:bg-neutral-900 dark:text-neutral-200",
            Navbar {}

            Outlet::<Route> {}
        }
    }
}

static MAIN_DROPDOWN_OPEN_STATE: GlobalSignal<bool> = Signal::global(|| false);

fn close_main_dropdown() {
    *MAIN_DROPDOWN_OPEN_STATE.write() = false;
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            onmouseleave: move |_| close_main_dropdown(),
            class: "flex flex-row space-x-4 p-2 items-center justify-between bg-gradient-to-b from-white to-neutral-100 dark:from-neutral-800 dark:to-neutral-900 border-b border-solid border-neutral-300 dark:border-neutral-950 text-neutral-600 dark:text-neutral-300",

            div { class: "w-1/4 text-2xl",
                MainNavDropdownMenu {}
                MainLinks {}
            }
            div { class: "w-1/2", Search {} }
            div { class: "w-1/4 text-right", MainSettings {} }
        }
    }
}

#[component]
fn MainLinks() -> Element {
    rsx! {
        span { class: "ml-2 text-lg",
            Link { to: Route::Home {}, "Boilermaker" }
            img { class: "inline h-6 w-6 mr-1", src: FAVICON }
        }
    }
}

#[component]
fn MainNavDropdownMenu() -> Element {
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
                onmouseleave: move |_| {
                    close_main_dropdown();
                },
                //TODO: put dropdown close in closure to avoid repetition
                onclick: move |_| {
                    close_main_dropdown();
                },
                class: "absolute left-0 top-13 w-48 bg-white dark:bg-neutral-900 rounded shadow-lg border border-l-0 border-t-0 border-neutral-300 dark:border-neutral-700 z-10 text-sm ",
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Home {},
                    i { class: "fa-solid fa-house" }
                    span { class: "ml-2", "Home" }
                }
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Templates {},
                    i { class: "fa-solid fa-note-sticky" }
                    span { class: "ml-2", "Templates" }
                }
                Link {
                    class: INDENTED_DROPDOWN_LINK_STYLE,
                    to: Route::TemplateNew {},
                    i { class: "fa-solid fa-plus" }
                    span { class: "ml-2", "New Template" }
                }
                Link { class: DROPDOWN_LINK_STYLE, to: Route::Docs {},
                    i { class: "fa-solid fa-file-code" }
                    span { class: "ml-2", "Docs" }
                }
                Link { class: DROPDOWN_LINK_STYLE, to: Route::GetInvolved {},
                    i { class: "fa-solid fa-hands-helping" }
                    span { class: "ml-2", "Get Involved" }
                }
            }
        }
    }
}

#[component]
pub fn MainSettings() -> Element {
    rsx! {
        a {
            class: "px-4 py-2 rounded hover:bg-neutral-100 dark:hover:bg-neutral-700",
            href: "/settings",
            i { class: "fa-solid fa-gear" }
            span { class: "ml-2", "Settings" }
        }
    }
}
