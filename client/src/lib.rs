mod input;
mod state;

use custom_elements::CustomElement;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run() {
    state::optional::ComponentWrapper::define("horfimbor-template-state");
    input::optional::ComponentWrapper::define("horfimbor-template-input");
}
