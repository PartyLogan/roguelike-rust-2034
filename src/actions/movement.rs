use crate::actors::Actor;

use super::{Action, ActionResult};

pub struct WalkAction {
    pub x: i32,
    pub y: i32,
}
impl WalkAction {
    pub fn new(x: i32, y: i32) -> Self {
        WalkAction { x, y }
    }
}
impl Action for WalkAction {
    fn execute(&self, actor: &mut Actor) -> ActionResult {
        actor.x += self.x as i32;
        actor.y += self.y as i32;
        ActionResult::success()
    }
}
