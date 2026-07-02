use horfimbor_client::EventStoreProps;
use horfimbor_client::input::send_command;
use horfimbor_client_derive::WebComponent;
use serde::Deserialize;
use std::rc::Rc;
use web_sys::HtmlInputElement;
use weblog::{console_error, console_info};
use yew::platform::spawn_local;
use yew::prelude::*;

use template_shared::command::{Delay, TemplateCommand};

#[derive(WebComponent)]
#[component(TemplateInput)]
#[derive(Default, Properties, PartialEq, Deserialize, Clone)]
pub struct TemplateInputProps {
    pub endpoint: String,
    pub id: String,
}

impl EventStoreProps for TemplateInputProps {
    fn endpoint(&self) -> &str {
        self.endpoint.as_ref()
    }

    fn path(&self) -> &str {
        "/input"
    }

    fn jwt(&self) -> &str {
        ""
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
}

const DEFAULT_TO_ADD: usize = 42;
const DEFAULT_DELAY: usize = 2;

#[derive(Eq, PartialEq, Clone)]
pub struct LocalData {
    endpoint: String,
    id: String,
    nb: usize,
    delay: usize,
    error: Option<String>,
}

#[function_component(TemplateInput)]
pub fn display_input(props: &TemplateInputProps) -> Html {
    let message = use_reducer(|| LocalData {
        endpoint: props.endpoint.clone(),
        id: props.id.clone(),
        nb: DEFAULT_TO_ADD,
        delay: DEFAULT_DELAY,
        error: None,
    });
    html!( <>
            <ContextProvider<LocalContext> context={message}>
                <SetNb />
                <SetDelay />
                <ErrorDisplay />
                <Sender />
            </ContextProvider<LocalContext>>

    </>)
}

pub type LocalContext = UseReducerHandle<LocalData>;

pub enum LocalAction {
    ChangeNb(usize),
    ChangeDelay(usize),
    SetError(Option<String>),
}

impl LocalData {
    pub fn get_command(&self) -> Option<TemplateCommand> {
        // add some check if needed

        if self.delay == 0 {
            Some(TemplateCommand::Add(self.nb))
        } else {
            Some(TemplateCommand::Delayed(Delay {
                delay: self.delay,
                to_add: self.nb,
            }))
        }
    }
}

impl Reducible for LocalData {
    type Action = LocalAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut reduced = (*self).clone();
        match action {
            LocalAction::ChangeNb(nb) => {
                reduced.nb = nb;
                reduced
            }
            LocalAction::ChangeDelay(delay) => {
                reduced.delay = delay;
                reduced
            }
            LocalAction::SetError(e) => {
                reduced.error = e;
                reduced
            }
        }
        .into()
    }
}

#[function_component(ErrorDisplay)]
fn error_display() -> Html {
    let Some(msg_ctx) = use_context::<LocalContext>() else {
        console_error!("no context");
        return html!(<></>);
    };

    match msg_ctx.error.clone() {
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

#[function_component(SetNb)]
fn local_data_setter() -> Html {
    let Some(msg_ctx) = use_context::<LocalContext>() else {
        console_error!("no context");
        return html!(<></>);
    };

    let value = msg_ctx.nb;

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        match input.value().parse::<usize>() {
            Ok(new_nb) => {
                msg_ctx.dispatch(LocalAction::ChangeNb(new_nb));
            }
            Err(_) => {
                msg_ctx.dispatch(LocalAction::SetError(Some("cannot parse nb".to_string())));
            }
        };
    });

    html! {
        <div>
            <label>{"to add"}
                <input type="number" {oninput} value={value.to_string()} />
            </label><br/>
        </div>
    }
}
#[function_component(SetDelay)]
fn local_data_setter() -> Html {
    let Some(msg_ctx) = use_context::<LocalContext>() else {
        console_error!("no context");
        return html!(<></>);
    };

    let value = msg_ctx.delay;

    let oninput = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        match input.value().parse::<usize>() {
            Ok(new_nb) => {
                msg_ctx.dispatch(LocalAction::ChangeDelay(new_nb));
            }
            Err(_) => {
                msg_ctx.dispatch(LocalAction::SetError(Some(
                    "cannot parse delay".to_string(),
                )));
            }
        };
    });

    html! {
        <div>
            <label>{"delay"}
                <input type="number" {oninput} value={value.to_string()} />
            </label>
        </div>
    }
}

#[function_component(Sender)]
fn sender() -> Html {
    let Some(msg_ctx) = use_context::<LocalContext>() else {
        console_error!("no context");
        return html!(<></>);
    };

    let props = TemplateInputProps {
        endpoint: msg_ctx.endpoint.clone(),
        id: msg_ctx.id.clone(),
    };

    if let Some(cmd) = msg_ctx.get_command() {
        let props = props.clone();
        let cmd = cmd.clone();
        let msg_ctx = msg_ctx.clone();
        let on_send_clicked = Callback::from(move |_| {
            let props = props.clone();
            let cmd = cmd.clone();
            let msg_ctx = msg_ctx.clone();
            spawn_local(async move {
                match send_command(&cmd, props).await {
                    Ok(resp) => {
                        if resp.ok() {
                            console_info!("sent !");
                        }
                    }
                    Err(e) => {
                        msg_ctx.dispatch(LocalAction::SetError(Some(e)));
                    }
                }
            });
        });

        return html! { <button id="btn-send" onclick={on_send_clicked}>{"Send"}</button> };
    }
    html! { <></> }
}

// TODO
// #[function_component(Reset)]
// fn reset(props: &TemplateInputProps) -> Html {
//     let err = use_atom::<LocalError>();
//     let endpoint = props.endpoint.clone();
//
//     let on_reset_clicked = Callback::from(move |_| {
//         let err = err.clone();
//         let endpoint = endpoint.clone();
//         spawn_local(async move {
//             let cmd = TemplateCommand::Reset;
//             let endpoint = endpoint.clone();
//
//             spawn_local(async move {
//                 match send_command(&cmd, endpoint).await {
//                     Ok(resp) => {
//                         if resp.ok() {
//                             console_info!("sent !");
//                         }
//                     }
//                     Err(e) => {
//                         err.set(LocalError { err: Some(e) });
//                     }
//                 }
//             });
//         });
//     });
//
//     html! { <button id="btn-reset" onclick={on_reset_clicked}>{"Reset"}</button> }
// }
