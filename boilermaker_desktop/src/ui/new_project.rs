use std::collections::HashMap;
use std::path::PathBuf;

use boilermaker_desktop::APP_STATE;
use color_eyre::Result;
use dioxus::prelude::*;

use crate::Route;
use boilermaker_core::commands::new as Command;
use boilermaker_core::db::TemplateResult;
use boilermaker_core::template as tpl;
use boilermaker_core::template::static_analysis::find_variables_in_path;
use boilermaker_views::{BTN_GREEN_STYLE, INPUT_STYLE, LABEL_STYLE, LINK_STYLE};

// TODO: Share relevant enums across UI elements. This is also used in TemplateAdd.
enum ResultMessage {
    None,
    Error(String),
    Success(String),
}

struct FormState {
    dir: String,
    overwrite: bool,
    rename: String,
    variables: Vec<String>,
    variable_values: HashMap<String, String>,
}

impl FormState {
    fn to_new_command(&self, name: &str) -> Command::New {
        let new_vars: Vec<String> = self
            .variable_values
            .iter()
            .map(|(key, val)| format!("{}={}", key, val))
            .collect();

        Command::New {
            name: name.to_owned(),
            dir: Some(self.dir.clone()).filter(|s| !s.trim().is_empty()),
            overwrite: self.overwrite,
            vars: new_vars,
            lang: None,
            rename: Some(self.rename.clone()).filter(|s| !s.trim().is_empty()),
            output_path: None,
        }
    }
}

#[component]
pub fn NewProject(i: usize) -> Element {
    // Get pre-loaded templates from global context.
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
        let mut form_state = use_signal(|| FormState {
            dir: "/tmp".to_owned(),
            rename: t.name.clone(),
            overwrite: false,
            variables: get_all_variables(&t),
            variable_values: get_default_values(&t).unwrap_or_default(),
        });

        let tpl_name = use_signal(|| t.name.clone());
        let mut result_message = use_signal(|| ResultMessage::None);

        rsx! {
            div {
                class: "py-4 px-2",
                h1 { class: "text-2xl text-neutral-500", "New Project: {t.name}" }

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

                form {
                    class: "p-4",
                    onsubmit: move |e| async move {
                        e.prevent_default();
                        let app_state = APP_STATE.get().expect("APP_STATE not initialized");
                        let command = form_state.read().to_new_command(tpl_name.read().as_ref());
                        match Command::new(app_state, &command).await {
                            Ok(_) => {
                                result_message
                                    .set(
                                        ResultMessage::Success(
                                            "Project created successfully!".to_string(),
                                        ),
                                    )
                            }
                            Err(err) => {
                                result_message
                                    .set(ResultMessage::Error(format!("Error creating project: {err}")))
                            }
                        }
                    },
                    div { class: "flex h-full w-full",
                        // Left pane: New command config.
                        div { class: "w-1/2 overflow-auto p-4",
                            h2 { class: "text-xl mb-4", "Configuration" }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-link" }
                                    span { class: "ml-2", "Work directory" }
                                }
                                input {
                                    name: "dir",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    value: form_state.read().dir.clone(),
                                    oninput: move |e| form_state.write().dir = e.value()
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-link" }
                                    span { class: "ml-2", "Project name" }
                                }
                                input {
                                    name: "rename",
                                    r#type: "text",
                                    class: INPUT_STYLE,
                                    value: form_state.read().rename.clone(),
                                    oninput: move |e| form_state.write().rename = e.value()
                                }
                            }
                            div { class: "mb-4",
                                label { class: LABEL_STYLE,
                                    i { class: "fa-solid fa-link" }
                                    span { class: "ml-2", "Overwrite?" }
                                }
                                input {
                                    name: "overwrite",
                                    r#type: "checkbox",
                                    checked: form_state.read().overwrite,
                                    class: INPUT_STYLE,
                                    oninput: move |e| form_state.write().overwrite = e.checked()
                                }
                            }
                        }
                        // Right pane: template variables.
                        div { class: "w-1/2 overflow-auto p-4",
                            h2 { class: "text-xl mb-4", "Template variables" }
                            for name in form_state.read().variables.clone() {
                                div { class: "mb-4",
                                    label { class: LABEL_STYLE,
                                        i { class: "fa-solid fa-link" }
                                        span { class: "ml-2", "{name}" }
                                    }
                                    input {
                                        name: "__var__{name}",
                                        r#type: "text",
                                        class: INPUT_STYLE,
                                        placeholder: "{name}",
                                        value: form_state.read().variable_values.get(&name).cloned().unwrap_or_default(),
                                        oninput: move |e| {
                                            form_state.write().variable_values.insert(name.clone(), e.value());
                                        }
                                    }
                                }
                            }
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

// Get all acceptable template variables for an existing DB Template.
// FIXME: this is probably a "core" utils function.
fn get_all_variables(t: &TemplateResult) -> Vec<String> {
    let mut sorted_variables: Vec<String> = find_variables_in_path(&t.template_dir)
        .unwrap_or_default()
        .into_iter()
        .collect();
    sorted_variables.sort();
    sorted_variables
}

// Read template's config. file for a list of default values.
// FIXME: this is probably a "core" utils function.
fn get_default_values(t: &TemplateResult) -> Result<HashMap<String, String>> {
    let base_dir = PathBuf::from(&t.template_dir);
    let tpl_config = tpl::get_template_config(&base_dir)?;
    let hm = tpl_config
        .variables
        .as_ref()
        .map(|vars| vars.as_map().clone())
        .unwrap_or_default();
    Ok(hm)
}
