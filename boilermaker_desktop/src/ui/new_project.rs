use dioxus::prelude::*;

use boilermaker_core::db::TemplateResult;
use boilermaker_core::template::static_analysis::find_variables_in_path;
use boilermaker_views::{BTN_GREEN_STYLE, INPUT_STYLE, LABEL_STYLE, LINK_STYLE};
use tracing::debug;

use crate::Route;

#[component]
pub fn NewProject(i: usize) -> Element {
    // Get pre-loaded templates from context.
    let tpl_context = use_context::<Signal<Vec<TemplateResult>>>();
    let templates = tpl_context.read();
    let content = if templates.is_empty() {
        rsx! {
            div { class: "py-4 text-neutral-500 dark:text-neutral-200",
                "No templates found. "
                Link { class: LINK_STYLE, to: Route::TemplateAdd {}, "Add some templates to get started!" }
            }
        }
    } else {
        let t = &templates[i];
        // TODO: Use template's context + "allowed" vars to build a form with default values.
        let vars = find_variables_in_path(&t.template_dir).unwrap_or_default();
        rsx! {
            div {
                class: "py-4 px-2",
                h1 { class: "text-2xl text-neutral-500", "New Project: {t.name}" }
                form {
                    class: "p-4",
                    onsubmit: move |_| { debug!("FIXME :)") },
                    for name in vars.iter() {
                        div { class: "mb-4",
                            label { class: LABEL_STYLE,
                                i { class: "fa-solid fa-link" }
                                span { class: "ml-2", "{name}" }
                            }
                            input {
                                name: "{name}",
                                r#type: "text",
                                class: INPUT_STYLE,
                                placeholder: "{name}"
                            }
                        }
                    }
                    div { class: "mb-4",
                        label { class: LABEL_STYLE,
                            i { class: "fa-solid fa-link" }
                            span { class: "ml-2", "Target directory" }
                        }
                        input {
                            name: "output_dir",
                            r#type: "text",
                            class: INPUT_STYLE,
                            placeholder: "/tmp"
                        }
                    }
                    div { class: "mb-6",
                        button { class: BTN_GREEN_STYLE, r#type: "submit", "Create project" }
                    }
                }
            }
        }
    };

    rsx! {
        document::Title { "Boilermaker" }
        { content }
    }
}
