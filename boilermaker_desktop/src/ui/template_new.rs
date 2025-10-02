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
    let mut status = use_signal(|| HashMap::<String, Option<(bool, String)>>::new());

    rsx! {
        document::Title { "Create New Template - Boilermaker" }

        div { class: "py-4 px-2",
            h1 { class: "text-2xl mb-4 px-4", "Add a new template" }

            div { class: "p-0 flex",
                div { class: "flex-grow p-4 rounded",
                    div { class: "p-0",
                        form { class: "p-4",
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-signature" }
                                    span { class: "ml-2", "Template Name" }
                                }
                                input {
                                    name: "name",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template name",
                                    oninput: move |e| name.set(e.value()),
                                    value: "{name}",
                                    onblur: move |_| {
                                        let name_val = name.read().trim().to_string();
                                        if !name_val.is_empty() {
                                            status
                                                .write()
                                                .insert(
                                                    "name".to_string(),
                                                    Some((true, "Template name is valid".to_string())),
                                                );
                                        } else {
                                            status
                                                .write()
                                                .insert(
                                                    "name".to_string(),
                                                    Some((false, "Name is required".to_string())),
                                                );
                                        }
                                    },
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Template Description (optional))" }
                                textarea {
                                    name: "description",
                                    class: TEXTAREA_STYLE,
                                    placeholder: "Enter a description for the template",
                                    oninput: move |e| description.set(e.value()),
                                    value: "{description}",
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Template Repository URL" }
                                input {
                                    name: "repo",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "e.g. https://github.com/yeajustmars/boilermaker",
                                    oninput: move |e| repo.set(e.value()),
                                    value: "{repo}",
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Git Repo Branch (optional)" }
                                input {
                                    name: "branch",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template branch (default: main)",
                                    oninput: move |e| branch.set(e.value()),
                                    value: "{branch}",
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE, "Git Repo Subdirectory (optional)" }
                                input {
                                    name: "subdir",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "e.g. /examples/hello-world (default: /)",
                                    oninput: move |e| subdir.set(e.value()),
                                    value: "{subdir}",
                                }
                            }
                            div { class: "mb-6",
                                button { class: BTN_CREATE_STYLE, r#type: "submit", "Add Template" }
                            }
                        }
                    }
                }
                div { class: "w-128 p-4 rounded border border-neutral-200 dark:border-neutral-700 mr-4",
                    h2 { class: "text-xl mb-4", "Template status" }
                    AddTemplateStatusSidebar { status: status.clone() }
                }
            }
        }
    }
}

#[component]
fn AddTemplateStatusSidebar(status: Signal<HashMap<String, Option<(bool, String)>>>) -> Element {
    #[rustfmt::skip]
    let keys = vec![
        ("Name",         "name",        "fa-solid fa-signature"),
        ("Description",  "description", "fa-solid fa-file-lines"),
        ("Repo URL",     "repo",        "fa-solid fa-link"),
        ("Branch",       "branch",      "fa-solid fa-code-branch"),
        ("Subdirectory", "subdir",      "fa-solid fa-folder"),
    ];

    rsx! {
        ul {
            for (label , key , icon) in keys {
                li { class: "mb-2",
                    div { class: "flex",
                        div { class: "w-1/3",
                            i { class: icon }
                            span { class: "italic pl-2", "{label}: " }
                        }
                        div { class: "w-3/4",
                            match status.read().get(key).cloned().flatten() {
                                Some((true, msg)) => rsx! {
                                    span { class: "text-green-500", "✅ {msg}" }
                                },
                                Some((false, msg)) => rsx! {
                                    span { class: "text-red-500", "💥 {msg}" }
                                },
                                None => rsx! {
                                    span { class: "italic text-gray-500", "Pending" }
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
