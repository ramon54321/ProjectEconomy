use super::{Action, ActionPayload, ActionResult};

pub struct ChatAction {
    step: u8,
}
impl ChatAction {
    pub fn new() -> Self {
        Self { step: 0 }
    }
}
impl Action for ChatAction {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult {
        println!("Chatting for the {} time", self.step);
        self.step = self.step + 1;
        if self.step < 4 {
            return ActionResult::InProgress;
        }
        ActionResult::Done(Box::new(ChatAction::new()))
    }
}
