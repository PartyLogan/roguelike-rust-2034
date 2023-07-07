use crate::actors::Actor;

pub mod movement;

pub trait Action {
    fn execute(&self, actor: &mut Actor) -> ActionResult;
}

pub struct ActionResult {
    pub success: bool,
    pub alternative: Option<Box<dyn Action>>,
}

impl ActionResult {
    pub fn new(result: bool, alternative: Option<Box<dyn Action>>) -> Self {
        ActionResult {
            success: result,
            alternative: None,
        }
    }

    pub fn success() -> ActionResult {
        ActionResult::new(true, None)
    }

    pub fn faulure() -> ActionResult {
        ActionResult::new(false, None)
    }

    pub fn alternative(action: Option<Box<dyn Action>>) -> ActionResult {
        ActionResult::new(false, action)
    }
}
