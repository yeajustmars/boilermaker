use std::collections::HashMap;

use dioxus::prelude::*;

use boilermaker_views::{BTN_CREATE_STYLE, INPUT_STYLE, LABEL_STYLE, TEXTAREA_STYLE};

#[component]
pub fn TemplateNew() -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut repo = use_signal(|| String::new());
    let mut branch = use_signal(|| String::new());
    let mut subdir = use_signal(|| String::new());
    let mut errors = use_signal(|| HashMap::<String, Option<String>>::new());

    rsx! {
        document::Title { "Create New Template - Boilermaker" }

        div { class: "py-4 px-2",
            h1 { class: "text-2xl mb-4", "Add a new template" }

            div { class: "p-0 flex",
                div { class: "flex-grow p-4 rounded",
                    div { class: "p-0",
                        form { class: "p-4",
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Template Name" }
                                input {
                                    name: "name",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template name",
                                    value: "{name}",
                                    onblur: move |_| {
                                        let name_val = name.read().trim().to_string();
                                        if name_val.is_empty() {
                                            errors
                                                .write()
                                                .insert("name".to_string(), Some("Name is required".to_string()));
                                        } else {
                                            errors.write().insert("name".to_string(), None);
                                        }
                                    },
                                    //validate_name(&e.data.value.clone()),
                                    oninput: move |e| name.set(e.value()),
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Template Description (optional))" }
                                textarea {
                                    class: TEXTAREA_STYLE,
                                    placeholder: "Enter a description for the template",
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Template Repository URL" }
                                input {
                                    name: "repo",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template repository URL",
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Git Repo Branch (optional)" }
                                input {
                                    name: "branch",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template branch (default: main)",
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Git Repo Subdirectory (optional)" }
                                input {
                                    name: "subdir",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template path (default: /)",
                                }
                            }
                            div { class: "mb-6",
                                button { class: BTN_CREATE_STYLE, r#type: "submit", "Create Template" }
                            }
                        }
                    }
                }
                div { class: "w-72 p-4 rounded border border-neutral-200 dark:border-neutral-700 mr-4",
                    h2 { class: "text-xl mb-4", "Template status" }
                    ul {
                        li { class: "mb-2",
                            div { class: "flex",
                                div { class: "w-1/3",
                                    i { class: "fa-solid fa-signature" }
                                    span { class: "italic", " Name: " }
                                }
                                div { class: "w-3/4",
                                    match errors.read().get("name").cloned().flatten() {
                                        Some(err) => rsx! {
                                            span { class: "text-red-400 pl-4", "💥 {err}" }
                                        },
                                        None if !name.read().is_empty() => rsx! {
                                            span { class: "text-green-600 pl-4", "✅" }
                                        },
                                        _ => rsx! {},
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
