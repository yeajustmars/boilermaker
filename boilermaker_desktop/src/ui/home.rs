use dioxus::prelude::*;
use tracing::error;

use boilermaker_desktop::APP_STATE;
use boilermaker_views::Echo;

#[component]
pub fn Home() -> Element {
    let resource = use_resource(move || async move {
        let cache = APP_STATE.get().unwrap().template_db.write().unwrap();
        match cache.list_templates().await {
            Ok(templates) => Ok(templates),
            Err(e) => {
                error!("Error fetching templates: {}", e);
                Err(e)
            }
        }
    });

    let result_signal = resource.suspend().with_loading_placeholder(|| {
        rsx! {
            div { "Loading templates..." }
        }
    });

    let th_style = "px-4 py-2 bg-gray-200 text-left";
    let td_style = "px-4 py-2";

    rsx! {
        document::Title { "Boilermaker" }
        div { class: "py-4 px-2",
            h1 { class: "text-3xl font-bold", "Latest Boilermaker Templates" }

            match result_signal {
                Err(e) => {
                    error!("Failed to load templates: {}", e);
                    rsx! {
                        div { class: "text-red-500", "Failed to load templates." }
                    }
                }
                Ok(signal) => {
                    let signal_value = signal.try_read_unchecked().unwrap();
                    let templates = signal_value.as_ref().unwrap();
                    if templates.is_empty() {
                        rsx! {
                            div { "No templates found. Add some templates to get started!" }
                        }
                    } else {
                        rsx! {
                            table {
                                thead {
                                    tr {
                                        th { class: th_style, "Name" }
                                        th { class: th_style, "Language" }
                                        th { class: th_style, "Repo" }
                                    }
                                }
                                tbody {
                                    for t in templates {
                                        tr {
                                            td { class: td_style, "{t.name}" }
                                            td { class: td_style, "{t.lang}" }
                                            td { class: td_style, "{t.repo}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Echo {}
    }
}
