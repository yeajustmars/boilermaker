use dioxus::prelude::{Element, VirtualDom};
use dioxus::ssr::Renderer;

/*
pub fn dioxus_to_html(app: fn() -> Element) -> String {
    let mut vdom = VirtualDom::new(app);
    vdom.rebuild_in_place();
    render(&vdom)
}
 */

pub fn dioxus_to_html_page(app: fn() -> Element) -> String {
    /*
    let mut vdom = VirtualDom::new(app);
    vdom.rebuild_in_place();
    let page_content = render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{page_content}</html>")
     */

    let mut vdom = VirtualDom::new(app);
    vdom.rebuild_in_place();

    let mut renderer = Renderer::new();
    renderer.pre_render = true;

    let page_content = renderer.render(&vdom);
    format!("<!DOCTYPE html><html lang='en'>{page_content}</html>")
}
