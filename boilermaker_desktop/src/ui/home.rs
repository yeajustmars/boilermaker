use dioxus::prelude::*;
use tracing::error;

use boilermaker_core::db::ListTemplateOptions;
use boilermaker_desktop::APP_STATE;
use boilermaker_views::{
    BTN_DELETE_STYLE, BTN_EDIT_STYLE, LINK_STYLE, TD_STYLE, TH_MUTED_STYLE, TH_STYLE,
};

use crate::Route;

#[component]
pub fn Home() -> Element {
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
            div { "Loading templates..." }
        }
    });

    rsx! {
        document::Title { "Boilermaker" }

        div { class: "py-4 px-2",
            h1 { class: "text-2xl text-neutral-500", "Latest Boilermaker Templates" }

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
                        rsx! {
                            table { class: "mt-6",
                                thead {
                                    tr {
                                        th {}
                                        th { class: TH_STYLE, "Name" }
                                        th { class: TH_STYLE, "Language" }
                                        th { class: TH_STYLE, "Repo" }
                                        th { class: TH_STYLE, "Subdirectory" }
                                        th { class: TH_MUTED_STYLE, "Actions" }
                                    }
                                }
                                tbody {
                                    for (i , t) in templates.iter().enumerate() {
                                        tr {
                                            td { class: "italic text-sm text-neutral-500", "{i + 1}" }
                                            td { class: TD_STYLE, "{t.name}" }
                                            td { class: TD_STYLE, "{t.lang}" }
                                            td { class: TD_STYLE, "{t.repo}" }
                                            td { class: TD_STYLE,
                                                match &t.subdir {
                                                    Some(subdir) => rsx! {
                                                    "{subdir}"
                                                    },
                                                    None => rsx! { "-" },
                                                }
                                            }
                                            td { class: TD_STYLE,
                                                div { class: "flex gap-2",
                                                    // TODO: Add global fn for creating buttons
                                                    button {
                                                        class: BTN_EDIT_STYLE,
                                                        "aria-label": "Edit Template",
                                                        i { class: "fas fa-edit" }
                                                    }
                                                    button {
                                                        class: BTN_DELETE_STYLE,
                                                        "aria-label": "Delete Template",
                                                        i { class: "fas fa-trash" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
