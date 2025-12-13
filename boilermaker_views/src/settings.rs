use dioxus::prelude::*;

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
