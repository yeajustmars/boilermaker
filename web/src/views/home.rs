use dioxus::prelude::*;

use indoc::indoc;

use crate::views::constants::{FAVICON, GITHUB_DARK_CSS, HIGHLIGHT_JS, LINK_STYLE};
use crate::views::Echo;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Title { "Boilermaker - Project Templates Made Easy" }
        //TODO: decide on switching between light/dark modes for code blocks
        // document::Stylesheet { href: GITHUB_LIGHT_CSS }
        document::Stylesheet { href: GITHUB_DARK_CSS }
        document::Script { src: "{HIGHLIGHT_JS}" }
        document::Script { "hljs.highlightAll();" }
        Hero {}
        div { class: "py-4 px-2",
            p {
                "Boilermaker is a thin project management system that helps you quickly set up programming projects"
                " with sensible defaults and configurations. It is written in Rust but the templates themselves are language-agnostic."
                "You either choose from one of the "
                Link { class: LINK_STYLE, to: Route::Templates {}, "public templates" }
                "or plug in your own for later reuse."
            }
            p { class: "mt-4",
                "Boilermaker is designed to be customizable, allowing you to adapt it to your specific needs."
                " Other than providing a framework for "
                Link { class: LINK_STYLE, to: "/structure", "structure" }
                ", "
                Link { class: LINK_STYLE, to: "/variables", "variable interpolation" }
                " and "
                Link { class: LINK_STYLE, to: "/configuration", "configuration" }
                ", it does not impose any opinions on how you should organize your code or project."
                " However, it does aim to provide a best practices approach to each language it has templates for."
            }
            Quickstart {}
        }
        Echo {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "h-48 p-10 text-center border-b border-solid border-neutral-300 dark:border-neutral-700
            bg-gradient-to-b from-white to-neutral-100 text-neutral-800
            dark:bg-gradient-to-b dark:from-neutral-800 dark:to-neutral-900 dark:text-neutral-300 
            flex flex-col justify-center items-center",
            h1 { class: "text-5xl font-bold",
                span {
                    "Welcome to Boilermaker!"
                    img { class: "inline h-10 w-10 ml-2", src: FAVICON }
                }
            }
            p { class: "mt-4 text-lg italic",
                "Making boilerplate less painful, or at least more manageable."
            }
        }
    }
}

#[component]
pub fn Quickstart() -> Element {
    let quickstart = indoc! {"
        # Install the CLI
        # TODO: detect OS and provide specific install instructions
        sudo apt install boilermaker

        # Create a new project
        boil new test-api --language=rust --template=axum-postgres

        # Run the project 
        cargo run
    "};

    rsx! {
        h2 { class: "mt-4 text-2xl", "Quickstart:" }

        pre { class: "bg-neutral-100 dark:bg-neutral-800 p-2 rounded mt-4 text-sm",
            code { class: "language-shell", "{quickstart}" }
        }
    }
}
