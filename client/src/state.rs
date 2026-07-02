use horfimbor_client::EventStoreProps;
use horfimbor_client::state::{AddEvent, EventStoreState};
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use yew::prelude::*;

use template_shared::dto::TemplateDto;
use template_shared::event::TemplateEvent;

type TemplateState = EventStoreState<TemplateDto, TemplateEvent, TemplateStateProps>;

#[derive(WebComponent)]
#[component(TemplateState)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct TemplateStateProps {
    pub endpoint: String,
    pub id: String,
}

impl EventStoreProps for TemplateStateProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    fn path(&self) -> &str {
        "data"
    }

    fn jwt(&self) -> &str {
        ""
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}

impl AddEvent<TemplateEvent, TemplateStateProps> for TemplateDto {
    fn play_event(&mut self, event: &TemplateEvent) {
        self.play_event(event);
    }

    fn get_view(&self, _props: TemplateStateProps) -> Html {
        html! {
            <div style="float:right">
                {"Average : ~"}{self.average()}<br/>
                <ul>
                { for self.last_ten().iter().map(|(c, n)| html!{
                    <li>{c}{n}</li>
                } )}
                </ul>
            </div>
        }
    }
}
