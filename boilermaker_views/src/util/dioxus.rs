use dioxus::prelude::{Element, VirtualDom};
use dioxus::ssr::render;

pub fn dioxus_to_html(app: fn() -> Element) -> String {
    let mut vdom = VirtualDom::new(app);
    vdom.rebuild_in_place();
    render(&vdom)
}

pub fn dioxus_to_html_page(app: fn() -> Element) -> String {
    let mut vdom = VirtualDom::new(app);
    vdom.rebuild_in_place();
    let page_content = render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{page_content}</html>")
}
