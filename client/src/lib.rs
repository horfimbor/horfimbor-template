mod input;
mod state;

use crate::input::TemplateInput;
use crate::state::TemplateState;
use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use weblog::console_info;
use yew::AppHandle;

// API that counts visits to the web-page
const API_BASE_URL: &str = "http://localhost:8000/api/";

#[derive(Default)]
struct ComponentWrapperState {
    content: Option<AppHandle<TemplateState>>,
}

impl CustomElement for ComponentWrapperState {
    fn inject_children(&mut self, this: &HtmlElement) {
        self.content =
            Some(yew::Renderer::<TemplateState>::with_root(this.clone().into()).render());
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

#[derive(Default)]
struct ComponentWrapperInput {
    content: Option<AppHandle<TemplateInput>>,
}

impl CustomElement for ComponentWrapperInput {
    fn inject_children(&mut self, this: &HtmlElement) {
        self.content =
            Some(yew::Renderer::<TemplateInput>::with_root(this.clone().into()).render());
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
    ComponentWrapperState::define("gyg-template-state");
    ComponentWrapperInput::define("gyg-template-input");
}
