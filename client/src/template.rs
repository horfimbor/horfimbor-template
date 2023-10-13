use bounce::BounceRoot;
use bounce::{use_atom, Atom};
use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use reqwasm::http::{Request, Response};
use serde::Deserialize;
use serde_json::Error;
use web_sys::HtmlInputElement;
use weblog::console_info;
use yew::platform::spawn_local;
use yew::prelude::*;

use template_shared::command::TemplateCommand;
use template_shared::dto::TemplateDto;
use template_shared::event::TemplateEvent;

// API that counts visits to the web-page
const API_BASE_URL: &str = "http://localhost:8000/api/";

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
pub struct Template {
    es: Option<EventSource>,
    dto: Result<TemplateDto, String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DtoMessage {
    Dto(TemplateDto),
    Event(TemplateEvent),
    Error(String),
}

impl Component for Template {
    type Message = DtoMessage;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut es = match EventSource::new(format!("{}data", API_BASE_URL).as_str()) {
            Ok(es) => es,
            Err(_) => {
                return Self {
                    es: None,
                    dto: Err(format!("cannot open eventsource to {}data", API_BASE_URL)),
                };
            }
        };

        let mut stream = match es.subscribe("message") {
            Ok(stream) => stream,
            Err(_) => {
                return Self {
                    es: None,
                    dto: Err(format!("cannot subscribe to all messages")),
                };
            }
        };

        let link = ctx.link().clone();
        spawn_local(async move {
            while let Some(Ok((_, msg))) = stream.next().await {
                if let Some(json) = msg.data().as_string() {
                    let message: Result<DtoMessage, Error> = serde_json::from_str(json.as_str());

                    let link = link.clone();
                    match message {
                        Ok(m) => {
                            link.send_message(m);
                        }
                        Err(_) => {
                            link.send_message(DtoMessage::Error(format!("stream closed")));
                        }
                    }
                }
            }
            link.send_message(DtoMessage::Error(format!("EventSource closed")));
            console_info!("EventSource Closed");
        });

        Self {
            es: Some(es),
            dto: Ok(TemplateDto::default()),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DtoMessage::Dto(d) => {
                self.dto = Ok(d);
                true
            }
            DtoMessage::Event(e) => match &mut self.dto {
                Ok(ref mut dto) => {
                    dto.play_event(&e);
                    true
                }
                Err(_) => false,
            },
            DtoMessage::Error(e) => {
                self.dto = Err(e);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let state = move || -> Html {
            match &self.dto {
                Ok(dto) => {
                    html! {
                        <div style="float:right">
                            {"Average : "}{dto.average()}<br/>
                            <ul>
                            { for dto.last_ten().iter().map(|(c, n)| html!{
                                <li>{c}{n}</li>
                            } )}
                            </ul>
                        </div>
                    }
                }
                Err(e) => {
                    html! {
                        <h2 style="float:right">
                            {e}
                        </h2>
                    }
                }
            }
        };

        html! {
            <BounceRoot>
                <fieldset>
                    {state()}
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
                </fieldset>
            </BounceRoot>
        }
    }
}
