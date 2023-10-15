use bounce::BounceRoot;
use bounce::{Atom, use_atom};
use reqwasm::http::{Request, Response};
use web_sys::HtmlInputElement;
use weblog::console_info;
use yew::platform::spawn_local;
use yew::prelude::*;

use template_shared::command::TemplateCommand;
use template_shared::dto::TemplateDto;
use crate::API_BASE_URL;

#[derive(Eq, PartialEq, Atom)]
struct LocalData {
    nb: Result<usize, String>,
}

const DEFAULT_TO_ADD: usize = 42;

impl Default for LocalData {
    fn default() -> Self {
        Self {
            nb: Ok(DEFAULT_TO_ADD),
        }
    }
}

#[function_component(ErrorDisplay)]
fn error_display() -> Html {
    let data = use_atom::<LocalData>();

    match data.nb.clone() {
        Ok(_) => {
            html! {}
        }
        Err(e) => {
            html! {
                <h2>
                    {e}
                </h2>
            }
        }
    }
}

#[function_component(LocalDataSetter)]
fn local_data_setter() -> Html {
    let data = use_atom::<LocalData>();

    let on_text_input = {
        let data = data.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            data.set(LocalData {
                nb: input
                    .value()
                    .parse()
                    .map_err(|_e| "cannot parse input".to_string()),
            });
        })
    };
    let nb = match data.nb {
        Ok(nb) => nb,
        Err(_) => DEFAULT_TO_ADD,
    }
    .to_string();
    html! {
        <div>
            <input type="number" oninput={on_text_input} value={nb} />
        </div>
    }
}

#[function_component(Sender)]
fn sender() -> Html {
    let data = use_atom::<LocalData>();

    let on_send_clicked = Callback::from(move |_| {
        let data = data.clone();
        let cmd = TemplateCommand::Add(data.nb.clone().unwrap_or(DEFAULT_TO_ADD));

        spawn_local(async move {
            match send_command(&cmd).await {
                Ok(resp) => {
                    if resp.ok() {
                        console_info!("sent !");
                    }
                }
                Err(e) => {
                    data.set(LocalData { nb: Err(e) });
                }
            }
        });
    });

    html! { <button id="btn-send" onclick={on_send_clicked}>{"Send"}</button> }
}

async fn send_command(cmd: &TemplateCommand) -> Result<Response, String> {
    Request::post(API_BASE_URL)
        .body(serde_json::to_string(&cmd).map_err(|_| format!("cannot serialize cmd {:?}", &cmd))?)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|_| "fail to send command".to_string())
}

#[function_component(Reset)]
fn reset() -> Html {
    let data = use_atom::<LocalData>();

    let on_reset_clicked = Callback::from(move |_| {
        let data = data.clone();
        spawn_local(async move {
            let cmd = TemplateCommand::Reset;

            spawn_local(async move {
                match send_command(&cmd).await {
                    Ok(resp) => {
                        if resp.ok() {
                            console_info!("sent !");
                        }
                    }
                    Err(e) => {
                        data.set(LocalData { nb: Err(e) });
                    }
                }
            });
        });
    });

    html! { <button id="btn-reset" onclick={on_reset_clicked}>{"Reset"}</button> }
}

#[derive(PartialEq, Atom, Default)]
struct State {
    content: TemplateDto,
}

#[allow(dead_code)]
pub struct TemplateInput {}

impl Component for TemplateInput {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BounceRoot>
                <div>
                    <LocalDataSetter />
                    <Sender />
                </div>
                <div>
                    <Reset />
                </div>
                <div>
                    <ErrorDisplay />
                </div>
            </BounceRoot>
        }
    }
}
