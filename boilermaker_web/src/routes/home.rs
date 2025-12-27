use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::Html};
use dioxus::prelude::*;

use boilermaker_views::{constants::LINK_STYLE, util::dioxus_to_html_page, web::HtmlLayout};

use crate::WebAppState;

pub async fn home(State(_app): State<Arc<WebAppState>>) -> Result<Html<String>, StatusCode> {
    let page = || {
        rsx! {
            HtmlLayout {
                HomePage {}
            }
        }
    };

    Ok(Html(dioxus_to_html_page(page)))
}

#[component]
pub fn HomePage() -> Element {
    rsx! {
        div { class: "w-full p-8",
            h1 { class: "text-2xl text-blue-400 mb-4 px-4 mt-4",
                "Welcome to Boilermaker!"
            }
            div { class: "mb-8 px-4",
                p { class: "p-4",
                    "Boilermaker is a language-agnostic project template framework "
                    "designed for rapid setup of common boilerplate. "
                    "It does this by providing ready-to-use (executable) templates and also by "
                    "allowing the user to easily add their own."
                }
                p { class: "p-4",
                    "Search our template sources by typing above in the search bar or"
                    a { href: "/docs/templates/new", class: LINK_STYLE,
                        "add your own locally"
                    }
                    "."
                }
            }
            div { class: "flex sm:flex-col md:flex-row gap-4",
                div { class: "sm:w-full md:w-1/4 p-4",
                    h2 { class: "text-2xl",
                        "Latest Templates"
                    }
                    p { class: "p-4",
                        "Check out the "
                        a { href: "/templates?order=desc&limit=20", class: LINK_STYLE,
                            "Templates"
                        }
                        " page to see the latest templates available."

                    }
                }
                div { class: "sm:w-full md:w-1/2 p-4",
                    LearnCard {}
                }
                div { class: "sm:w-full md:w-1/4 p-4",
                    h2 { class: "text-2xl",
                        "Get Involved!"
                    }
                    p { class: "p-4",
                        "The Boilermaker project is always looking for maintainers."
                        "If that's something you're interested in, please "
                        a { href: "/contact", class: LINK_STYLE, "contact us" }
                        "."
                    }
                }
            }
        }
    }
}

const HELLO_WORLD_RUST_EXAMPLE: &str = "
boil install https://github.com/yeajustmars/boilermaker \\
    -d examples/hello-world \\
    -n hello-world-rs

boil new hello-world-rs -d /tmp

cd /tmp/hello-world-rs

cargo run";

const HELLO_WORLD_CLOJURE_EXAMPLE: &str = "
boil install https://github.com/yeajustmars/hello-world-clj \\

boil new hello-world-clj -d /tmp

cd /tmp/hello-world-clj

bin/test

bin/repl";

#[component]
pub fn LearnCard() -> Element {
    rsx! {
        h2 { class: "text-2xl",
            "Learn Boilermaker"
        }
        p { class: "p-4",
            "See the "
            a { href: "/docs/getting-started",
                class: LINK_STYLE,
                "Getting Started"
            }
            " guide to get up-and-running quickly."
        }

        h3 { class: "text-xl mt-2 text-blue-400",
            a { href: "/docs/installation",
                class: LINK_STYLE,
                "Installation"
            }
        }
        pre { class: "bg-gray-800 text-white p-4 rounded",
            code { class: "language-shell",
                "curl -sSL https://boilermaker.dev/install.sh | bash"
            }
        }

        h3 { class: "text-xl mt-2 text-blue-400",
            a { href: "/docs/hello-world/rust",
                class: LINK_STYLE,
                "Hello World in Rust"
            }
        }
        span { class: "text-xs text-gray-400 ml-2",
            "(requires Rust installed)"
        }
        pre { class: "bg-gray-800 text-white p-4 rounded",
            code { class: "language-shell",
                "{HELLO_WORLD_RUST_EXAMPLE}"
            }
        }

        h3 { class: "text-xl mt-2 text-blue-400",
            a { href: "/docs/hello-world/clojure",
                class: LINK_STYLE,
                "Hello World in Clojure"
            }
        }
        span { class: "text-xs text-gray-400 ml-2",
            "(requires Clojure CLI Tools installed)"
        }
        pre { class: "bg-gray-800 text-white p-4 rounded",
            code { class: "language-shell",
                "{HELLO_WORLD_CLOJURE_EXAMPLE}"
            }
        }

    }
}
