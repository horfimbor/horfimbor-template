use bounce::BounceRoot;
use bounce::{use_atom, use_atom_value, Atom};
use reqwasm::http::Request;
use template_shared::command::TemplateCommand;
use web_sys::{ HtmlInputElement};
use weblog::console_info;
use yew::platform::spawn_local;
use yew::prelude::*;
use gloo_net::eventsource::futures::EventSource;
use futures::StreamExt;
use serde::Deserialize;
use template_shared::dto::TemplateDto;
use template_shared::event::TemplateEvent;


// API that counts visits to the web-page
const API_BASE_URL: &str = "http://localhost:8000/api/";

#[derive(Eq, PartialEq, Atom)]
struct LocalData {
    nb: usize,
}

impl Default for LocalData {
    fn default() -> Self {
        Self { nb: 42 }
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
                nb: input.value().parse().unwrap(),
            });
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
                console_info!("sent !");
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


#[derive(PartialEq, Atom, Default)]
struct State {
    content : TemplateDto
}

#[function_component(StateGetter)]
fn state_getter() -> Html {
    let data = use_atom_value::<State>();

    let nb = data.content.average();

    html! { <em>{nb}</em> }
}


pub struct Template {
  es : EventSource
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DtoMessage {
    Dto(TemplateDto),
    Event(TemplateEvent)
}

impl Component for Template {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {

        let mut es = EventSource::new(format!("{}data", API_BASE_URL).as_str()).unwrap();
        let mut stream = es.subscribe("message").unwrap();


        spawn_local(async move {
            // let state = use_atom::<State>();

            let mut tmpDto = TemplateDto::default();
            while let Some(Ok((_, msg))) = stream.next().await {
                if let Some(json) = msg.data().as_string(){
                    let message: DtoMessage = serde_json::from_str(json.as_str()).unwrap();
                    console_info!(format!("message ::: {:?}", message));

                    match message {
                        DtoMessage::Dto(dto) => {
                            tmpDto = dto.clone();
                            // state.set(
                            //     State{
                            //         content: dto
                            //     }
                            // ).unwrap()
                        }
                        DtoMessage::Event(event) => {
                            tmpDto.play_event(&event);
                            // state.set(
                            //     State{
                            //         content: tmpDto.clone()
                            //     }
                            // ).unwrap()
                        }
                    };
                    console_info!(format!("state ::: {:?}", tmpDto));
                }
            }
            console_info!("EventSource Closed");
        });

        Self {
            es
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BounceRoot>
                <fieldset>
                    <div style="float:right">
                        <StateGetter />
                    </div>
                    <div>
                        <LocalDataSetter />
                        <Sender />
                    </div>
                    <div>
                        <Resetter />
                    </div>

                </fieldset>
            </BounceRoot>
        }
    }
}
