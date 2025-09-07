use dioxus::prelude::*;

use views::{
    constants::{FAVICON, MAIN_CSS, TAILWIND_CSS},
    Docs, GetInvolved, Home, Search, Templates,
};

mod views;

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
        #[route("/get-involved")]
        GetInvolved {},
}

fn main() {
    dioxus::launch(App);
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
        document::Stylesheet { href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css" }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Roboto:ital,wght@0,100..900;1,100..900&display=swap" }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Fira+Code:wght@300;400;500;600;700&display=swap" }
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

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            onmouseleave: move |_| {
                *MAIN_DROPDOWN_OPEN_STATE.write() = false;
            },
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
    let link_style = "block px-4 py-2 hover:bg-neutral-100 dark:hover:bg-neutral-700";

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
                    *MAIN_DROPDOWN_OPEN_STATE.write() = false;
                },
                class: "absolute left-0 top-13 w-48 bg-white dark:bg-neutral-900 rounded shadow-lg border border-l-0 border-t-0 border-neutral-300 dark:border-neutral-700 z-10 text-sm ",
                Link { class: link_style, to: Route::Home {},
                    i { class: "fa-solid fa-house" }
                    span { class: "ml-2", "Home" }
                }
                Link { class: link_style, to: Route::Templates {},
                    i { class: "fa-solid fa-note-sticky" }
                    span { class: "ml-2", "Templates" }
                }
                Link { class: link_style, to: Route::Docs {},
                    i { class: "fa-solid fa-file-code" }
                    span { class: "ml-2", "Docs" }
                }
                Link { class: link_style, to: Route::GetInvolved {},
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
            href: "/login",
            i { class: "fa-solid fa-user" }
            span { class: "ml-2", "Login" }
        }
    }
}
