
mod template;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use weblog::console_info;
use yew::AppHandle;
use crate::template::Template;

#[derive(Default)]
struct ComponentWrapper {
    content: Option<AppHandle<Template>>,
}

impl CustomElement for ComponentWrapper {
    fn inject_children(&mut self, this: &HtmlElement) {
        self.content = Some(yew::Renderer::<Template>::with_root(this.clone().into()).render());
    }

    fn shadow() -> bool {
        false
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_info!("connected template");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_info!("disconnected template !");
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapper::define("gyg-template");
}
