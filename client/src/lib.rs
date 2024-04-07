mod input;
mod state;

use crate::input::{TemplateInput, TemplateInputProps};
use crate::state::{TemplateState, TemplateStateProps};
use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use weblog::console_info;
use yew::AppHandle;

// API that counts visits to the web-page

#[derive(Default)]
struct ComponentWrapperState {
    content: Option<AppHandle<TemplateState>>,
}

impl CustomElement for ComponentWrapperState {
    fn inject_children(&mut self, this: &HtmlElement) {
        let props = TemplateStateProps {
            endpoint: this.get_attribute("endpoint").unwrap_or_default(),
        };

        self.content = Some(
            yew::Renderer::<TemplateState>::with_root_and_props(this.clone().into(), props)
                .render(),
        );
    }

    fn shadow() -> bool {
        false
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_info!("connected template state");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_info!("disconnected template state!");
    }
}

#[derive(Default)]
struct ComponentWrapperInput {
    content: Option<AppHandle<TemplateInput>>,
}

impl CustomElement for ComponentWrapperInput {
    fn inject_children(&mut self, this: &HtmlElement) {
        let props = TemplateInputProps {
            endpoint: this.get_attribute("endpoint").unwrap_or_default(),
        };

        self.content = Some(
            yew::Renderer::<TemplateInput>::with_root_and_props(this.clone().into(), props)
                .render(),
        );
    }

    fn shadow() -> bool {
        false
    }

    fn connected_callback(&mut self, _this: &HtmlElement) {
        console_info!("connected template input");
    }

    fn disconnected_callback(&mut self, _this: &HtmlElement) {
        console_info!("disconnected template input!");
    }
}

#[wasm_bindgen]
pub fn run() {
    ComponentWrapperState::define("horfimbor-template-state");
    ComponentWrapperInput::define("horfimbor-template-input");
}
