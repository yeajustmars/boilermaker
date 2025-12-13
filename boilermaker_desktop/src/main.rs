use color_eyre::eyre::Result;
use dioxus::prelude::*;

mod templates_context;
mod ui;

use boilermaker_desktop::{init_app_state, APP_STATE};
use boilermaker_views::{
    DesktopSearch, Docs, GetInvolved, MainSettings, Templates, DROPDOWN_LINK_STYLE, FAVICON,
    FONT_AWESOME_URL, FONT_FIRA_CODE_URL, FONT_ROBOTO_URL, INDENTED_DROPDOWN_LINK_STYLE,
    LAYOUT_STYLE, MAIN_CSS, NAVBAR_STYLE, TAILWIND_CSS,
};
use templates_context::init_templates_context;
pub use templates_context::TemplatesContext;
use ui::{Home, NewProject, TemplateAdd};

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
        TemplateAdd {},
        #[route("/projects/new/:i")]
        NewProject {i: usize},
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
    init_templates_context();

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
        document::Stylesheet { href: FONT_AWESOME_URL }
        document::Stylesheet { href: FONT_ROBOTO_URL }
        document::Stylesheet { href: FONT_FIRA_CODE_URL}
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
            // TODO: move layout tailwind to views::constants
            class: LAYOUT_STYLE,
            Navbar {}

            Outlet::<Route> {}
        }
    }
}

static MAIN_DROPDOWN_OPEN_STATE: GlobalSignal<bool> = Signal::global(|| false);

fn close_main_dropdown() {
    *MAIN_DROPDOWN_OPEN_STATE.write() = false;
}

// TODO: [question] move Navbar to boilermaker_views?
// NOTE: It's possible these need to behave differently in desktop vs web views as the
// web view is static HTML, doesn't rely on a Dioxus signals, etc.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            onmouseleave: move |_| close_main_dropdown(),
            class: NAVBAR_STYLE,

            div { class: "w-1/4 text-2xl",
                MainNavDropdownMenu {}
                MainLinks {}
            }
            div { class: "w-1/2", DesktopSearch {} }
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
            }
        }
    }
}
