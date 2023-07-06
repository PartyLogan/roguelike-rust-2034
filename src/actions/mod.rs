use crate::actors::Actor;

pub mod movement;

pub trait Action {
    fn execute(&self, actor: &mut Actor) -> Option<Box<dyn Action>>;
}
