use cucumber::{World, given, then, when};
use horfimbor_eventsource::{Dto, State};
use template_shared::command::Delay;
use template_shared::command::TemplateCommand::{Add, Delayed, Finalize, Reset};
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
fn add_to_template(world: &mut TemplateWorld, nb: usize) {
    play_result(world, world.model.try_command(Add(nb)));
}

#[when(regex = r"^i try to delay (\d+) by (\d+) secs")]
fn delay_template(world: &mut TemplateWorld, nb: usize, delay: usize) {
    play_result(
        world,
        world
            .model
            .try_command(Delayed(Delay { delay, to_add: nb })),
    );
}

#[when(regex = r"^i try to reset it$")]
fn reset_template(world: &mut TemplateWorld) {
    play_result(world, world.model.try_command(Reset));
}

#[when(regex = r"^i wait (\d+) seconds$")]
fn wait_template(world: &mut TemplateWorld, nb: u64) {
    world.model.time_pass(nb);
}
#[when(regex = r"^i try to finalize the delay (\d+)$")]
fn finalize_template(world: &mut TemplateWorld, nb: usize) {
    let id = world.model.get_id(nb);
    play_result(world, world.model.try_command(Finalize(id)));
}

#[then(regex = r"^it got a value of (\d+)$")]
fn result(world: &mut TemplateWorld, nb: usize) {
    assert_eq!(nb, world.model.get_value());
}

#[then(regex = r"^it got an error$")]
fn error(world: &mut TemplateWorld) {
    assert!(world.err.is_some());
}

#[then(regex = r"^remaining delay is (\d+)$")]
fn remaining(world: &mut TemplateWorld, nb: usize) {
    assert_eq!(nb, world.model.delayed().len());
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
