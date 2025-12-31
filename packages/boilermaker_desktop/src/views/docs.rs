// desktop::docs

// TODO: link specific docs through router?
// use crate::Route;

use dioxus::prelude::*;

#[component]
pub fn Docs() -> Element {
    rsx! {
        h1 { "Docs" }

        /*
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Docs { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Docs { id: id + 1 },
                "Next"
            }
        }
         */
    }
}
