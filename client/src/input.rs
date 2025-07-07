use bounce::BounceRoot;
use bounce::{Atom, use_atom};
use horfimbor_client_derive::WebComponent;
use reqwasm::http::{Request, Response};
use web_sys::HtmlInputElement;
use weblog::console_info;
use yew::platform::spawn_local;
use yew::prelude::*;

use template_shared::command::{Delay, TemplateCommand};
use template_shared::dto::TemplateDto;

#[derive(WebComponent)]
#[component(TemplateInput)]
#[derive(Default, Properties, PartialEq)]
pub struct TemplateInputProps {
    pub endpoint: String,
}

const DEFAULT_TO_ADD: usize = 42;
const DEFAULT_DELAY: usize = 2;

#[derive(Eq, PartialEq, Atom)]
struct LocalData {
    nb: usize,
    delay: usize,
}

impl Default for LocalData {
    fn default() -> Self {
        Self {
            nb: DEFAULT_TO_ADD,
            delay: DEFAULT_DELAY,
        }
    }
}

#[derive(Eq, PartialEq, Atom, Default)]
struct LocalError {
    err: Option<String>,
}

#[function_component(ErrorDisplay)]
fn error_display() -> Html {
    let data = use_atom::<LocalError>();

    match data.err.clone() {
        None => {
            html! {}
        }
        Some(e) => {
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
    let err = use_atom::<LocalError>();

    let on_nb_input = {
        let data = data.clone();
        let err = err.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            let nb: Result<usize, String> = input
                .value()
                .parse()
                .map_err(|_e| "cannot parse input nb".to_string());

            match nb {
                Ok(nb) => {
                    data.set(LocalData {
                        nb,
                        delay: data.delay,
                    });
                    err.set(LocalError { err: None });
                }
                Err(s) => err.set(LocalError { err: Some(s) }),
            };
        })
    };

    let on_delay_input = {
        let data = data.clone();
        let err = err;

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();

            let delay: Result<usize, String> = input
                .value()
                .parse()
                .map_err(|_e| "cannot parse input delay".to_string());

            match delay {
                Ok(delay) => {
                    data.set(LocalData { nb: data.nb, delay });
                    err.set(LocalError { err: None });
                }
                Err(s) => err.set(LocalError { err: Some(s) }),
            };
        })
    };

    html! {
        <div>
            <label>{"to add"}
                <input type="number" oninput={on_nb_input} value={data.nb.to_string()} />
            </label><br/>
            <label>{"delay"}
                <input type="number" oninput={on_delay_input} value={data.delay.to_string()} />
            </label>
        </div>
    }
}

#[function_component(Sender)]
fn sender(props: &TemplateInputProps) -> Html {
    let data = use_atom::<LocalData>();
    let err = use_atom::<LocalError>();
    let endpoint = props.endpoint.clone();

    let on_send_clicked = Callback::from(move |_| {
        let data = data.clone();
        let err = err.clone();

        let cmd = if data.delay == 0 {
            TemplateCommand::Add(data.nb)
        } else {
            TemplateCommand::Delayed(Delay {
                delay: data.delay,
                to_add: data.nb,
            })
        };

        let endpoint = endpoint.clone();
        spawn_local(async move {
            let endpoint = endpoint.clone();
            match send_command(&cmd, endpoint).await {
                Ok(resp) => {
                    if resp.ok() {
                        console_info!("sent !");
                    }
                }
                Err(e) => {
                    err.set(LocalError { err: Some(e) });
                }
            }
        });
    });

    html! { <button id="btn-send" onclick={on_send_clicked}>{"Send"}</button> }
}

async fn send_command(cmd: &TemplateCommand, endpoint: String) -> Result<Response, String> {
    Request::post(endpoint.as_str())
        .body(serde_json::to_string(&cmd).map_err(|_| format!("cannot serialize cmd {:?}", &cmd))?)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|_| "fail to send command".to_string())
}

#[function_component(Reset)]
fn reset(props: &TemplateInputProps) -> Html {
    let err = use_atom::<LocalError>();
    let endpoint = props.endpoint.clone();

    let on_reset_clicked = Callback::from(move |_| {
        let err = err.clone();
        let endpoint = endpoint.clone();
        spawn_local(async move {
            let cmd = TemplateCommand::Reset;
            let endpoint = endpoint.clone();

            spawn_local(async move {
                match send_command(&cmd, endpoint).await {
                    Ok(resp) => {
                        if resp.ok() {
                            console_info!("sent !");
                        }
                    }
                    Err(e) => {
                        err.set(LocalError { err: Some(e) });
                    }
                }
            });
        });
    });

    html! { <button id="btn-reset" onclick={on_reset_clicked}>{"Reset"}</button> }
}

#[allow(dead_code)]
#[derive(PartialEq, Atom, Default)]
struct State {
    content: TemplateDto,
}

#[allow(dead_code)]
pub struct TemplateInput {}

impl Component for TemplateInput {
    type Message = ();

    type Properties = TemplateInputProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let endpoint = ctx.props().endpoint.clone();
        html! {
            <BounceRoot>
                <div>
                    <LocalDataSetter />
                    <Sender endpoint={endpoint.clone()} />
                </div>
                <div>
                    <Reset endpoint={endpoint.clone()} />
                </div>
                <div>
                    <ErrorDisplay />
                </div>
            </BounceRoot>
        }
    }
}
