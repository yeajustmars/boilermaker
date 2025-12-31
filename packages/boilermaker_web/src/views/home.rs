use dioxus::prelude::*;

use boilermaker_ui::Echo;

#[component]
pub fn Home() -> Element {
    rsx! {
        Echo {}
    }
}
