use dioxus::prelude::*;

use indoc::indoc;

use crate::views::constants::{FAVICON, GITHUB_DARK_CSS, GITHUB_LIGHT_CSS, HIGHLIGHT_JS};
use crate::views::Echo;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: GITHUB_LIGHT_CSS }
        document::Stylesheet { href: GITHUB_DARK_CSS }
        Hero {}
        div { class: "py-4 px-2",
            p {
                "Boilermaker is a small template management system that helps you quickly set up programming projects"
                " with sensible defaults and configurations. It is written in Rust but is otherwise language-agnostic."
                "You either choose from one of the "
                Link { class: "text-blue-400 px-1", to: Route::Templates {}, "public templates" }
                "or plug in your own for later reuse."
            }
            p { class: "mt-4",
                "Boilermaker is designed to be customizable, allowing you to adapt it to your specific needs."
                " Other than providing a simple structure and variable interpolation and configuration "
                " frameworks, it does not impose any opinions on how you should organize your code or project."
            }
            Quickstart {}
        }
        Echo {}
        script { src: "{HIGHLIGHT_JS}" }
        script { "hljs.highlightAll();" }
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            class: "h-48 p-10 text-neutral-300 text-center border-b border-solid border-neutral-300 dark:border-neutral-700 bg-gradient-to-b from-neutral-800 to-neutral-900 flex flex-col justify-center items-center",
            h1 { class: "text-5xl font-bold",
                span {
                    "Welcome to Boilermaker!"
                    img { class: "inline h-10 w-10 ml-2", src: FAVICON }
                
                }
            }
            p { class: "mt-4 text-lg italic",
                "We like to think we're easing the burden of setting up boilerplate in code projects!"
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
