use dioxus::prelude::*;

use boilermaker_views::{
    BTN_BLUE_STYLE, BTN_GREEN_STYLE, BTN_RED_STYLE, LINK_STYLE, TD_STYLE, TH_MUTED_STYLE, TH_STYLE,
};

use crate::{Route, TemplatesContext};

#[component]
pub fn Home() -> Element {
    // Get pre-loaded templates from context.
    let templates_ctx = use_context::<TemplatesContext>();
    let templates = templates_ctx.templates.read();
    let content = if templates.is_empty() {
        rsx! {
            div { class: "py-4 text-neutral-500 dark:text-neutral-200",
                "No templates found. "
                Link { class: LINK_STYLE, to: Route::TemplateAdd {}, "Add some templates to get started!" }
            }
        }
    } else {
        let navigator = use_navigator();
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
                                        class: BTN_GREEN_STYLE,
                                        "aria-label": "New Project",
                                        onclick:  move |_| { navigator.push(Route::NewProject { i }); },
                                        i { class: "fas fa-plus" },
                                    }
                                    button {
                                        class: BTN_BLUE_STYLE,
                                        "aria-label": "Template Details",
                                        i { class: "fas fa-eye" }
                                    }
                                    button {
                                        class: BTN_RED_STYLE,
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
    };

    rsx! {
        document::Title { "Boilermaker" }
        div { class: "py-4 px-2",
            h1 { class: "text-2xl text-neutral-500", "My Templates" }
            { content }
        }
    }
}
