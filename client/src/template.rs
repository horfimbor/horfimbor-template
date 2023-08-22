use web_sys::HtmlInputElement;
use yew::prelude::*;
use bounce::{use_atom, Atom, use_atom_value};
use bounce::BounceRoot;
use reqwasm::http::Request;
use weblog::console_info;
use yew::platform::spawn_local;
use template_shared::command::TemplateCommand;

// API that counts visits to the web-page
const API_BASE_URL: &str = "http://localhost:8000/api/";


#[derive(Eq, PartialEq, Atom)]
struct LocalData {
    nb: usize,
}

impl Default for LocalData{
    fn default() -> Self {
        Self{
            nb: 42,
        }
    }
}


pub struct Template {}


#[function_component(LocalDataSetter)]
fn local_data_setter() -> Html {
    let data = use_atom::<LocalData>();

    let on_text_input = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            data.set(LocalData { nb: input.value().parse().unwrap() });
        })
    };

    html! {
        <div>
            <input type="number" oninput={on_text_input} value={data.nb.to_string()} />
        </div>
    }
}

#[function_component(Sender)]
fn sender() -> Html {
    let data = use_atom_value::<LocalData>();

    let on_send_clicked = Callback::from(move |_| {

        let cmd = TemplateCommand::Add(data.nb);

        spawn_local(async move {
            let resp = Request::post(API_BASE_URL)
                .body(serde_json::to_string(&cmd).unwrap())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap();

            if resp.ok() {
                console_info!("reset !");
            }
        });

    });

    html! { <button id="btn-reset" onclick={on_send_clicked}>{"Send"}</button> }
}

#[function_component(Resetter)]
fn resetter() -> Html {

    let on_reset_clicked = Callback::from(move |_| {
        spawn_local(async move {

            let cmd = TemplateCommand::Reset;

            let resp = Request::post(API_BASE_URL)
                .body(serde_json::to_string(&cmd).unwrap())
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap();

            if resp.ok() {
                console_info!("reset !");
            }
        });

    });

    html! { <button id="btn-reset" onclick={on_reset_clicked}>{"Reset"}</button> }
}

impl Component for Template {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <BounceRoot>
                <p>
                    <div>
                        <LocalDataSetter />
                        <Sender />
                    </div>
                    <hr/>
                    <div>
                        <Resetter />
                    </div>
                </p>
            </BounceRoot>
        }
    }
}
