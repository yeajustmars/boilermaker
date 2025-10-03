use std::collections::HashMap;

use dioxus::prelude::*;
use git2;
use tokio::runtime::Runtime;

use boilermaker_boil::commands::add::{add, Add}; // TODO: move actual cmds to Core
use boilermaker_core::constants::{BRANCH_PATTERN, SUBDIR_PATTERN};
use boilermaker_desktop::APP_STATE;
use boilermaker_views::{BTN_CREATE_STYLE, INPUT_STYLE, LABEL_STYLE, TEXTAREA_STYLE};

type SignalStringType = Signal<String>;
type StatusHashType = HashMap<String, Option<(bool, String)>>;
type StatusSignalType = Signal<StatusHashType>;

// TODO: add select to choose whether to overwrite
#[component]
pub fn TemplateNew() -> Element {
    let mut template = use_signal(|| String::new());
    let mut branch = use_signal(|| String::new());
    let mut subdir = use_signal(|| String::new());
    let mut lang = use_signal(|| String::new());
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut status = use_signal(|| HashMap::<String, Option<(bool, String)>>::new());

    rsx! {
        document::Title { "Create New Template - Boilermaker" }

        div { class: "py-4 px-2",
            h1 { class: "text-2xl mb-4 px-4", "Add a new template" }

            div { class: "p-0 flex",
                div { class: "flex-grow p-4 rounded",
                    div { class: "p-0",
                        form { class: "p-4", onsubmit: add_template,
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
                                    onblur: move |e| validate_template(e, &template, &mut status),
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
                                button { class: BTN_CREATE_STYLE, r#type: "submit", "Add Template" }
                            }
                        }
                    }
                }
                div { class: "w-128 p-4 rounded border border-neutral-200 dark:border-neutral-800 mr-4",
                    h2 { class: "text-xl mb-4", "Status" }
                    AddTemplateStatusSidebar { status: status.clone() }
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
        set_status(status, "repo", false, "Repo URL is required");
        return;
    }

    let remote = git2::Remote::create_detached(tpl_val.clone());
    if let Err(e) = remote {
        set_status(status, "repo", false, &format!("Invalid repo URL: {}", e));
        return;
    }

    let mut remote = remote.unwrap();
    match remote.connect(git2::Direction::Fetch) {
        Ok(_) => {}
        Err(e) => {
            set_status(status, "repo", false, &format!("Invalid repo URL: {}", e));
            return;
        }
    }

    set_status(status, "repo", true, "is valid");
}

pub fn validate_branch(
    _event: Event<FocusData>,
    signal: &SignalStringType,
    status: &mut StatusSignalType,
) {
    let branch_val = sigval(&signal);
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
    let subdir_val = sigval(&signal);
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

pub fn add_template(e: Event<FormData>) {
    e.prevent_default();
    let add_args = e.to_add();

    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async { add(APP_STATE, add_args).await });
    println!("FormData: {:?}", e);
    println!("Add Args: {:?}", add_args);
}

trait AsOption {
    fn as_option(&self) -> Option<String>;
}

impl AsOption for FormValue {
    fn as_option(&self) -> Option<String> {
        let s = self.as_value();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}

trait ToAdd {
    fn to_add(&self) -> Add;
}

impl ToAdd for Event<FormData> {
    fn to_add(&self) -> Add {
        let values = &self.data.values();
        let template = values.get("template").unwrap().as_value();
        let branch = values.get("branch").unwrap().as_option();
        let subdir = values.get("subdir").unwrap().as_option();
        let lang = values.get("lang").unwrap().as_option();
        let name = values.get("name").unwrap().as_option();
        //let description = values.get("description").unwrap().as_option();

        Add {
            template,
            name,
            lang,
            branch,
            subdir,
            overwrite: true,
        }
    }
}
