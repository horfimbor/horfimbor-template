use futures::StreamExt;
use gloo_net::eventsource::futures::EventSource;
use serde::Deserialize;
use serde_json::Error;
use weblog::console_info;
use yew::platform::spawn_local;
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
}

#[derive(Default, Properties, PartialEq)]
pub struct TemplateStateProps {
    pub endpoint: String,
}

impl Component for TemplateState {
    type Message = DtoMessage;
    type Properties = TemplateStateProps;

    fn create(ctx: &Context<Self>) -> Self {
        let endpoint = ctx.props().endpoint.clone();

        let mut es = match EventSource::new(format!("{endpoint}data").as_str()) {
            Ok(es) => es,
            Err(_) => {
                return Self {
                    es: None,
                    dto: Err(format!("cannot open eventsource to {endpoint}data")),
                };
            }
        };

        let mut stream = match es.subscribe("message") {
            Ok(stream) => stream,
            Err(_) => {
                return Self {
                    es: None,
                    dto: Err("cannot subscribe to all messages".to_string()),
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
                            link.send_message(DtoMessage::Error("stream closed".to_string()));
                        }
                    }
                }
            }
            link.send_message(DtoMessage::Error("EventSource closed".to_string()));
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
            {state()}
        }
    }
}
