
use yew::prelude::*;

// API that counts visits to the web-page
// const API_BASE_URL: &str = "http://localhost:8000/api";

pub struct Template {}


impl Component for Template {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>
                { "Hello from yew" }
            </p>
        }
    }
}