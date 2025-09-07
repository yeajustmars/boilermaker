use dioxus::prelude::*;

use api;

#[component]
pub fn Search() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        input {
            id: "search",
            class: "block w-full rounded-md py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 dark:ring-neutral-700 placeholder:text-gray-700 dark:placeholder:text-gray-100 focus:ring-2 focus:ring-inset focus:ring-neutral-600 sm:text-sm sm:leading-6 px-2 border border-neutral-300 dark:border-neutral-700 bg-white dark:bg-neutral-900",
            placeholder: "Search...",
            oninput: move |event| async move {
                let data = api::search(event.value()).await.unwrap();
                response.set(data);
            },
        }
    }
}
