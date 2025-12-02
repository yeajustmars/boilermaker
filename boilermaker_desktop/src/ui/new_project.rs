use boilermaker_core::db::ListTemplateOptions;
use boilermaker_desktop::APP_STATE;
use boilermaker_views::LINK_STYLE;
use dioxus::prelude::*;
use tracing::error;

use crate::Route;

#[component]
pub fn NewProject(i: usize) -> Element {
    // FIXME: refactor this list into shared state.
    let resource = use_resource(move || async move {
        let cache = &APP_STATE.get().unwrap().template_db;
        let list_opts = Some(ListTemplateOptions {
            order_by: Some("created_at DESC, name ASC".to_string()),
            limit: Some(10),
            offset: None,
        });
        match cache.list_templates(list_opts).await {
            Ok(templates) => Ok(templates),
            Err(e) => {
                error!("Error fetching templates: {}", e);
                Err(e)
            }
        }
    });

    let result_signal = resource.suspend().with_loading_placeholder(|| {
        rsx! {
            div { "Loading template..." }
        }
    });

    rsx! {
        document::Title { "Boilermaker" }

        div { class: "py-4 px-2",
            h1 { class: "text-2xl text-neutral-500", "New Project" }

            match result_signal {
                Err(e) => {
                    error!("Failed to load templates: {}", e);
                    rsx! {
                        div { class: "text-red-400", "Failed to load templates." }
                    }
                }
                Ok(signal) => {
                    let signal_value = signal.try_read_unchecked().unwrap();
                    let templates = signal_value.as_ref().unwrap();
                    if templates.is_empty() {
                        rsx! {
                            div { class: "py-4 text-neutral-500 dark:text-neutral-200",
                                "No templates found. "
                                Link { class: LINK_STYLE, to: Route::TemplateAdd {}, "Add some templates to get started!" }
                            }
                        }
                    } else {
                        let t = &templates[i];
                        rsx!{
                            div { "New project with template {t.name}!" }
                        }
                    }
                }
            }
        }
    }
}
