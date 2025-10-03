use std::collections::HashMap;

use dioxus::prelude::*;
use git2;
use regex::Regex;

use boilermaker_boil::commands::add::{add, Add};
use boilermaker_views::{BTN_CREATE_STYLE, INPUT_STYLE, LABEL_STYLE, TEXTAREA_STYLE};

// TODO: add select to choose whether to overwrite
#[component]
pub fn TemplateNew() -> Element {
    let mut name = use_signal(|| String::new());
    let mut lang = use_signal(|| String::new());
    let mut repo = use_signal(|| String::new());
    let mut branch = use_signal(|| String::new());
    let mut subdir = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut status = use_signal(|| HashMap::<String, Option<(bool, String)>>::new());

    // TODO: move to common, reusable location
    let validate_name = move |_event| {
        status
            .write()
            .insert("name".to_string(), Some((true, "is valid".to_string())));
    };

    // TODO: move to common, reusable location
    let validate_lang = move |_event| {
        status
            .write()
            .insert("lang".to_string(), Some((true, "is valid".to_string())));
    };

    // TODO: check that the repo is able to be cloned
    // TODO: move to common, reusable location
    let validate_repo = move |_event| {
        let repo_val = repo.read().trim().to_string();

        if repo_val.is_empty() {
            status.write().insert(
                "repo".to_string(),
                Some((false, "Repo URL is required".to_string())),
            );
            return;
        }

        let remote = git2::Remote::create_detached(repo_val.clone());
        if let Err(e) = remote {
            status.write().insert(
                "repo".to_string(),
                Some((false, format!("Invalid repo URL: {}", e))),
            );
            return;
        }

        let mut remote = remote.unwrap();
        match remote.connect(git2::Direction::Fetch) {
            Ok(_) => {}
            Err(e) => {
                status.write().insert(
                    "repo".to_string(),
                    Some((false, format!("Invalid repo URL: {}", e))),
                );
                return;
            }
        }

        status
            .write()
            .insert("repo".to_string(), Some((true, "is valid".to_string())));
    };

    // TODO: move to constants or somewhere common
    let branch_pattern = Regex::new(r"^(refs/heads/)?[A-Za-z0-9._/-]+$").unwrap();
    // TODO: move to common, reusable location
    let validate_branch = move |_event| {
        let branch_val = branch.read().trim().to_string();
        if branch_val.is_empty() || branch_pattern.is_match(&branch_val) {
            status
                .write()
                .insert("branch".to_string(), Some((true, "is valid".to_string())));
        } else {
            status.write().insert(
                "branch".to_string(),
                Some((false, "Invalid branch name".to_string())),
            );
        }
    };

    // TODO: move to constants or somewhere common
    let subdir_pattern = Regex::new(r"^/?[A-Za-z0-9/\-_].*$").unwrap();
    // TODO: move to common, reusable location
    let validate_subdir = move |_event| {
        let subdir_val = subdir.read().trim().to_string();
        if subdir_val.is_empty() || subdir_pattern.is_match(&subdir_val) {
            status
                .write()
                .insert("subdir".to_string(), Some((true, "is valid".to_string())));
        } else {
            status.write().insert(
                "subdir".to_string(),
                Some((false, "Invalid path".to_string())),
            );
        }
    };

    // TODO: move to common, reusable location
    let validate_description = move |_event| {
        status.write().insert(
            "description".to_string(),
            Some((true, "is valid".to_string())),
        );
    };

    rsx! {
        document::Title { "Create New Template - Boilermaker" }

        div { class: "py-4 px-2",
            h1 { class: "text-2xl mb-4 px-4", "Add a new template" }

            div { class: "p-0 flex",
                div { class: "flex-grow p-4 rounded",
                    div { class: "p-0",
                        form {
                            class: "p-4",
                            onsubmit: move |e| {
                                e.prevent_default();
                                let add_cmd = Add {
                                    template: repo.read().trim().to_string(),
                                    name: if name.read().trim().is_empty() {
                                        None
                                    } else {
                                        Some(name.read().trim().to_string())
                                    },
                                    lang: None,
                                    branch: if branch.read().trim().is_empty() {
                                        None
                                    } else {
                                        Some(branch.read().trim().to_string())
                                    },
                                    subdir: if subdir.read().trim().is_empty() {
                                        None
                                    } else {
                                        Some(subdir.read().trim().to_string())
                                    },
                                    overwrite: true,
                                };
                                println!("Add command: {:?}", add_cmd);
                            },
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-link" }
                                    span { class: "ml-2", "Template Repo URL" }
                                }
                                input {
                                    name: "repo",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "e.g. https://github.com/yeajustmars/boilermaker",
                                    oninput: move |e| repo.set(e.value()),
                                    value: "{repo}",
                                    onblur: validate_repo,
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-signature" }
                                    span { class: "ml-2", "Template Name (optional)" }
                                }
                                input {
                                    name: "name",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template name",
                                    oninput: move |e| name.set(e.value()),
                                    value: "{name}",
                                    onblur: validate_name,
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-language" }
                                    span { class: "ml-2", "Template Language (optional)" }
                                }
                                input {
                                    name: "lang",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template language",
                                    oninput: move |e| lang.set(e.value()),
                                    value: "{lang}",
                                    onblur: validate_lang,
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-code-branch" }
                                    span { class: "ml-2", "Git Repo Branch (optional)" }
                                }
                                input {
                                    name: "branch",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template branch (default: main)",
                                    oninput: move |e| branch.set(e.value()),
                                    value: "{branch}",
                                    onblur: validate_branch,
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-folder" }
                                    span { class: "ml-2", "Git Repo Subdirectory (optional)" }
                                }
                                input {
                                    name: "subdir",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "e.g. /examples/hello-world (default: /)",
                                    oninput: move |e| subdir.set(e.value()),
                                    value: "{subdir}",
                                    onblur: validate_subdir,
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-file-lines" }
                                    span { class: "ml-2", "Template Description (optional))" }
                                }
                                textarea {
                                    name: "description",
                                    class: TEXTAREA_STYLE,
                                    placeholder: "Enter a description for the template",
                                    oninput: move |e| description.set(e.value()),
                                    value: "{description}",
                                    onblur: validate_description,
                                }
                            }
                            div { class: "mb-6",
                                button { class: BTN_CREATE_STYLE, r#type: "submit", "Add Template" }
                            }
                        }
                    }
                }
                div { class: "w-128 p-4 rounded border border-neutral-200 dark:border-neutral-800 mr-4",
                    h2 { class: "text-xl mb-4", "New template status" }
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
        ("Repo URL",     "repo",        "fa-solid fa-link"),
        ("Name",         "name",        "fa-solid fa-signature"),
        ("Lang",         "lang",        "fa-solid fa-language"),
        ("Branch",       "branch",      "fa-solid fa-code-branch"),
        ("Subdirectory", "subdir",      "fa-solid fa-folder"),
        ("Description",  "description", "fa-solid fa-file-lines"),
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
                                    span { class: "text-green-500", "{msg} ✅" }
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
