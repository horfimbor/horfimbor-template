mod input;
mod state;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    yew::set_event_bubbling(false);
    state::optional::ComponentWrapper::define("horfimbor-template-state");
    input::optional::ComponentWrapper::define("horfimbor-template-input");
}
