use color_eyre::eyre::Result;
use dioxus::prelude::*;

use boilermaker_desktop::{init_app_state, init_templates_context};
use boilermaker_ui::{Layout, Navbar, DROPDOWN_LINK_STYLE, INDENTED_DROPDOWN_LINK_STYLE};
use views::{Docs, GetInvolved, Home, NewProject, TemplateAdd, Templates};
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopLayout)]
        #[route("/")]
        Home {},
        #[route("/templates")]
        Templates {},
        #[route("/templates/new")]
        TemplateAdd {},
        #[route("/projects/new/:i")]
        NewProject {i: usize},
        #[route("/docs")]
        Docs {},
        #[route("/get-involved")]
        GetInvolved {},
}

fn main() -> Result<()> {
    init_app_state()?;
    dioxus::launch(App);
    Ok(())
}

#[component]
fn App() -> Element {
    init_templates_context();

    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn DesktopLayout() -> Element {
    rsx! {
        Layout {
            Navbar {
                Link { to: Route::Home {},
                    class: DROPDOWN_LINK_STYLE,
                    i { class: "fa-solid fa-house" }
                    span { class: "ml-2", "Home" }
                }
                Link { to: Route::Templates {},
                    class: DROPDOWN_LINK_STYLE,
                    i { class: "fa-solid fa-note-sticky" }
                    span { class: "ml-2", "Templates" }
                }
                Link { to: Route::TemplateAdd {},
                    class: INDENTED_DROPDOWN_LINK_STYLE,
                    i { class: "fa-solid fa-plus" }
                    span { class: "ml-2", "Add Template" }
                }
                Link { to: Route::Docs {},
                    class: DROPDOWN_LINK_STYLE,
                    i { class: "fa-solid fa-file-code" }
                    span { class: "ml-2", "Docs" }
                }
                Link { to: Route::GetInvolved {},
                    class: DROPDOWN_LINK_STYLE,
                    i { class: "fa-solid fa-hands-helping" }
                    span { class: "ml-2", "Get Involved" }
                }
            }
            Outlet::<Route> {}
        }
    }
}
