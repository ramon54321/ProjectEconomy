use super::{Action, ActionPayload, ActionResult};

pub struct IdleAction {
    step: u8,
}
impl IdleAction {
    pub fn new() -> Self {
        Self { step: 0 }
    }
}
impl Action for IdleAction {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult {
        // What do I have
        // What do I need
        // What do I want
        // How am I going to get it
        // Do it

        ActionResult::InProgress
    }
}
