use crate::actors::Actor;

use super::Action;

pub struct WalkAction {
    pub x: i32,
    pub y: i32,
}
impl Action for WalkAction {
    fn execute(&self, actor: &mut Actor) -> Option<Box<dyn Action>> {
        actor.x += self.x as i32;
        actor.y += self.y as i32;
        None
    }
}
