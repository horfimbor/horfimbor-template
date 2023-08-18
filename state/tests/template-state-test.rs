use cucumber::{given, then, when, World};
use gyg_eventsource::State;
use template_shared::command::TemplateCommand::*;
use template_shared::error::TemplateError;
use template_shared::event::TemplateEvent;
use template_state::TemplateState;

#[derive(World, Debug, Default)]
pub struct TemplateWorld {
    model: TemplateState,
    err: Option<TemplateError>,
}

#[given(regex = r"^a template$")]
fn a_template(world: &mut TemplateWorld) {
    world.model = TemplateState::default();
}

#[when(regex = r"^i try to add (\d+)")]
fn add_too_template(world: &mut TemplateWorld, nb: usize) {
    play_result(world, world.model.try_command(Add(nb)));
}

#[when(regex = r"^i try to reset it$")]
fn reset_template(world: &mut TemplateWorld) {
    play_result(world, world.model.try_command(Reset));
}

#[then(regex = r"^it got a value of (\d+)$")]
fn result(world: &mut TemplateWorld, nb: usize) {
    assert_eq!(nb, world.model.get_value())
}

#[then(regex = r"^it got a error$")]
fn error(world: &mut TemplateWorld) {
    assert!(world.err.is_some())
}

fn play_result(world: &mut TemplateWorld, events: Result<Vec<TemplateEvent>, TemplateError>) {
    match events {
        Ok(list) => {
            for e in list {
                world.model.play_event(&e);
            }
        }
        Err(e) => world.err = Some(e),
    }
}

#[tokio::main]
async fn main() {
    TemplateWorld::run("tests/book").await;
}
