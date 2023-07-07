use crate::{actors::Actor, level::Level, util::get_xy};

use super::{Action, ActionResult};

pub struct BumpAction {
    pub x: i32,
    pub y: i32,
}
impl BumpAction {
    pub fn new(x: i32, y: i32) -> Self {
        BumpAction { x, y }
    }
}

impl Action for BumpAction {
    fn execute(&self, actor: &mut Actor, level: &mut Level) -> ActionResult {
        let index = get_xy(actor.x + self.x, actor.y + self.y, level.width);
        let tile = level.get_tile(index);
        if !tile.passable {
            return ActionResult::faulure();
        }

        return ActionResult::alternative(Some(Box::new(WalkAction::new(self.x, self.y))));
    }
}

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
    fn execute(&self, actor: &mut Actor, level: &mut Level) -> ActionResult {
        actor.x += self.x as i32;
        actor.y += self.y as i32;
        ActionResult::success()
    }
}
