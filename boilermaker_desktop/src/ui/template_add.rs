use std::collections::HashMap;

use dioxus::prelude::*;

use crate::TemplatesContext;
use boilermaker_core::commands::install::{install, Install};
use boilermaker_core::constants::{BRANCH_PATTERN, SUBDIR_PATTERN};
use boilermaker_core::util::string::string_to_option;
use boilermaker_desktop::APP_STATE;
use boilermaker_views::{BTN_GREEN_STYLE, INPUT_STYLE, LABEL_STYLE, PRELOADER, TEXTAREA_STYLE};

type SignalStringType = Signal<String>;
type StatusHashMapType = HashMap<String, Option<(bool, String)>>;
type StatusSignalType = Signal<StatusHashMapType>;

enum ResultMessage {
    None,
    Error(String),
    Success(String),
}

#[component]
pub fn TemplateAdd() -> Element {
    let mut template = use_signal(String::new);
    let mut branch = use_signal(String::new);
    let mut subdir = use_signal(String::new);
    let mut lang = use_signal(String::new);
    let mut name = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut status = use_signal(StatusHashMapType::new);
    let mut processing = use_signal(|| false);
    let mut result_message = use_signal(|| ResultMessage::None);
    let mut templates_ctx = use_context::<TemplatesContext>();

    rsx! {
        document::Title { "Create New Template - Boilermaker" }

        div { class: "py-4 px-2 relative h-screen w-screen",
            h1 { class: "text-2xl mb-4 px-4", "Add a new template" }

            if *processing.read() {
                div { class: "absolute w-full h-full top-0 left-0 right-0 bottom-0 bg-black bg-opacity-75 z-50",
                    div { class: "fixed inset-0 flex items-center justify-center",
                        div { class: "text-center text-neutral-400 text-2xl",
                            div {
                                img { class: "", src: PRELOADER }
                                "Processing..."
                            }
                        }
                    }
                }
            }

            div { class: "py-2 px-4 text-left",
                match &*result_message.read() {
                    ResultMessage::None => rsx! {},
                    ResultMessage::Error(msg) => rsx! {
                        div { class: "text-center text-red-400 text-lg", "{msg}" }
                    },
                    ResultMessage::Success(msg) => rsx! {
                        div { class: "text-center text-green-400 text-2xl", "{msg}" }
                    },
                }
            }

            div { class: "p-0 flex",
                div { class: "flex-grow p-4 rounded",
                    div { class: "p-0",
                        form {
                            class: "p-4",
                            onsubmit: move |e| async move {
                                processing.set(true);
                                let app_state = APP_STATE.get().expect("APP_STATE not initialized");
                                let data = Install {
                                    template: sigval(&template),
                                    branch: string_to_option(&sigval(&branch)),
                                    subdir: string_to_option(&sigval(&subdir)),
                                    lang: string_to_option(&sigval(&lang)),
                                    name: string_to_option(&sigval(&name)),
                                };
                                match install(app_state, &data).await {
                                    Ok(_) => {
                                        result_message
                                            .set(
                                                ResultMessage::Success(
                                                    "Template added successfully!".to_string(),
                                                ),
                                            );
                                        templates_ctx.refresh();
                                    }
                                    Err(err) => {
                                        result_message
                                            .set(ResultMessage::Error(format!("Error adding template: {err}")))
                                    }
                                }
                                processing.set(false);
                                e.prevent_default();
                            },
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-link" }
                                    span { class: "ml-2", "Repo URL" }
                                }
                                input {
                                    name: "template",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "e.g. https://github.com/yeajustmars/boilermaker",
                                    oninput: move |e| template.set(e.value()),
                                    value: "{template}",
                                    onblur: move |e| async move { validate_template(e, &template, &mut status) },
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-code-branch" }
                                    span { class: "ml-2", "Repo Branch (optional)" }
                                }
                                input {
                                    name: "branch",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template branch (default: main)",
                                    oninput: move |e| branch.set(e.value()),
                                    value: "{branch}",
                                    onblur: move |e| validate_branch(e, &branch, &mut status),
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-folder" }
                                    span { class: "ml-2", "Repo Subdirectory (optional)" }
                                }
                                input {
                                    name: "subdir",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "e.g. /examples/hello-world (default: /)",
                                    oninput: move |e| subdir.set(e.value()),
                                    value: "{subdir}",
                                    onblur: move |e| validate_subdir(e, &subdir, &mut status),
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-language" }
                                    span { class: "ml-2", "Language (optional)" }
                                }
                                input {
                                    name: "lang",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template language",
                                    oninput: move |e| lang.set(e.value()),
                                    value: "{lang}",
                                    onblur: move |e| validate_lang(e, &lang, &mut status),
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-signature" }
                                    span { class: "ml-2", "Name (optional)" }
                                }
                                input {
                                    name: "name",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    placeholder: "Enter template name",
                                    oninput: move |e| name.set(e.value()),
                                    value: "{name}",
                                    onblur: move |e| validate_name(e, &name, &mut status),
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
                                    onblur: move |e| validate_description(e, &description, &mut status),
                                }
                            }
                            div { class: "mb-6",
                                button { class: BTN_GREEN_STYLE, r#type: "submit", "Add Template" }
                            }
                        }
                    }
                }
                div { class: "w-128 p-4 rounded border border-neutral-200 dark:border-neutral-800 mr-4",
                    h2 { class: "text-xl mb-4", "Status" }
                    AddTemplateStatusSidebar { status: status }
                }
            }
        }
    }
}

#[component]
fn AddTemplateStatusSidebar(status: StatusSignalType) -> Element {
    #[rustfmt::skip]
    let keys = vec![
        ("Repo URL",     "template",    "fa-solid fa-link"),
        ("Branch",       "branch",      "fa-solid fa-code-branch"),
        ("Subdirectory", "subdir",      "fa-solid fa-folder"),
        ("Lang",         "lang",        "fa-solid fa-language"),
        ("Name",         "name",        "fa-solid fa-signature"),
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

pub fn sigval(v: &SignalStringType) -> String {
    v.read().trim().to_string()
}

pub fn set_status(status: &mut StatusSignalType, key: &str, valid: bool, msg: &str) {
    status
        .write()
        .insert(key.to_string(), Some((valid, msg.to_string())));
}

pub fn validate_name(
    _event: Event<FocusData>,
    _signal: &SignalStringType,
    status: &mut StatusSignalType,
) {
    set_status(status, "name", true, "is valid");
}

pub fn validate_lang(
    _event: Event<FocusData>,
    _signal: &SignalStringType,
    status: &mut StatusSignalType,
) {
    set_status(status, "lang", true, "is valid");
}

// TODO: check that the repo is able to be cloned
pub fn validate_template(
    _event: Event<FocusData>,
    signal: &SignalStringType,
    status: &mut StatusSignalType,
) {
    let tpl_val = sigval(signal);

    if tpl_val.is_empty() {
        set_status(status, "template", false, "Repo URL is required");
        return;
    }

    let remote = git2::Remote::create_detached(tpl_val.clone());
    if let Err(e) = remote {
        set_status(status, "template", false, &format!("Invalid repo URL: {e}"));
        return;
    }

    let mut remote = remote.unwrap();
    match remote.connect(git2::Direction::Fetch) {
        Ok(_) => {}
        Err(e) => {
            set_status(status, "template", false, &format!("Invalid repo URL: {e}"));
            return;
        }
    }

    set_status(status, "template", true, "is valid");
}

pub fn validate_branch(
    _event: Event<FocusData>,
    signal: &SignalStringType,
    status: &mut StatusSignalType,
) {
    let branch_val = sigval(signal);
    if branch_val.is_empty() || BRANCH_PATTERN.is_match(&branch_val) {
        set_status(status, "branch", true, "is valid");
    } else {
        set_status(status, "branch", false, "Invalid branch name");
    }
}

pub fn validate_subdir(
    _event: Event<FocusData>,
    signal: &SignalStringType,
    status: &mut StatusSignalType,
) {
    let subdir_val = sigval(signal);
    if subdir_val.is_empty() || SUBDIR_PATTERN.is_match(&subdir_val) {
        set_status(status, "subdir", true, "is valid");
    } else {
        set_status(status, "subdir", false, "Invalid subdirectory path");
    }
}

pub fn validate_description(
    _event: Event<FocusData>,
    _signal: &SignalStringType,
    status: &mut StatusSignalType,
) {
    set_status(status, "description", true, "is valid");
}
