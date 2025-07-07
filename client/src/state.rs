use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use serde_json::Error;
use std::time::Duration;
use weblog::console_info;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

use template_shared::dto::TemplateDto;
use template_shared::event::TemplateEvent;

#[allow(dead_code)]
pub struct TemplateState {
    es: Option<EventSource>,
    dto: Result<TemplateDto, String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DtoMessage {
    Dto(TemplateDto),
    Event(TemplateEvent),
    Error(String),
    Reconnect,
}

#[derive(WebComponent)]
#[component(TemplateState)]
#[derive(Default, Properties, PartialEq)]
pub struct TemplateStateProps {
    pub endpoint: String,
}

impl TemplateState {
    fn connect(&mut self, ctx: &Context<Self>) {
        if self.es.is_some() {
            return;
        }
        self.dto = Ok(TemplateDto::default());

        let endpoint = ctx.props().endpoint.clone();

        let mut es = match EventSource::new(format!("{endpoint}data").as_str()) {
            Ok(es) => es,
            Err(_) => {
                self.dto = Err(format!("cannot open eventsource to {endpoint}data"));
                return;
            }
        };

        let mut stream = match es.subscribe("message") {
            Ok(stream) => stream,
            Err(_) => {
                self.dto = Err("cannot subscribe to all messages".to_string());
                return;
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
                            link.send_message(DtoMessage::Error("stream closed".to_string()));
                        }
                    }
                }
            }
            link.send_message(DtoMessage::Error("EventSource closed".to_string()));
            console_info!("EventSource Closed");
        });

        self.es = Some(es);
    }
}

impl Component for TemplateState {
    type Message = DtoMessage;
    type Properties = TemplateStateProps;

    fn create(ctx: &Context<Self>) -> Self {
        let mut state = Self {
            es: None,
            dto: Ok(TemplateDto::default()),
        };

        state.connect(ctx);

        state
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DtoMessage::Dto(d) => {
                self.dto = Ok(d);
                true
            }
            DtoMessage::Event(e) => match &mut self.dto {
                Ok(dto) => {
                    dto.play_event(&e);
                    true
                }
                Err(_) => false,
            },
            DtoMessage::Error(e) => {
                self.dto = Err(e);
                self.es = None;

                let link = ctx.link().clone();

                spawn_local(async move {
                    sleep(Duration::from_secs(5)).await;
                    link.send_message(DtoMessage::Reconnect);
                });
                true
            }
            DtoMessage::Reconnect => {
                self.connect(ctx);
                if self.dto.is_err() {
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        sleep(Duration::from_secs(5)).await;
                        link.send_message(DtoMessage::Reconnect);
                    });
                }
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
            {state()}
        }
    }
}
