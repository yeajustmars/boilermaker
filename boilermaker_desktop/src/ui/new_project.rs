use boilermaker_desktop::APP_STATE;
use dioxus::prelude::*;

use crate::Route;
use boilermaker_core::commands::new::{new, New};
use boilermaker_core::db::TemplateResult;
use boilermaker_core::template::static_analysis::find_variables_in_path;
use boilermaker_views::{BTN_GREEN_STYLE, INPUT_STYLE, LABEL_STYLE, LINK_STYLE};
use tracing::debug;

// TODO: share the enums across elements.
enum ResultMessage {
    None,
    Error(String),
    Success(String),
}

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
        // TODO: Use template's config + "allowed" vars to set default values.
        let vars = find_variables_in_path(&t.template_dir).unwrap_or_default();

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
                        let command = e.to_new(tpl_name.read().as_ref());
                        debug!("Boil command: {:?}", command);
                        match new(app_state, &command).await {
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
                    for name in vars.iter() {
                        div { class: "mb-4",
                            label { class: LABEL_STYLE,
                                i { class: "fa-solid fa-link" }
                                span { class: "ml-2", "{name}" }
                            }
                            input {
                                name: "__var__{name}",
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
                            name: "dir",
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

trait ToNew {
    fn to_new(&self, name: &str) -> New;
}

impl ToNew for Event<FormData> {
    fn to_new(&self, name: &str) -> New {
        let mut new_vars: Vec<String> = vec![];
        let form_values = &self.data.values();

        // Turn input vars into something that New.vars understands.
        for (key, val) in form_values.iter() {
            if !key.starts_with("__var__") {
                continue;
            }
            if let Some(str) = val.as_ref().first() {
                let key_unprefixed = &key[7..];
                let var = format!("{key_unprefixed}={}", str);
                new_vars.push(var);
            }
        }

        // Template dest dir: dir/template-name
        let dir = form_values
            .get("dir")
            .and_then(|vals| vals.first())
            .map(|v| v.to_owned())
            .filter(|s| !s.trim().is_empty());

        New {
            name: name.to_owned(),
            dir: dir,
            overwrite: false,
            vars: new_vars,
            lang: None,
            rename: None,
            output_path: None,
        }
    }
}
