use super::{accounting::account::Account, book::Book, store::Store};
use std::{cell::RefCell, rc::Weak};

pub(super) mod chat_action;

pub(super) enum ActionResult {
    InProgress,
    Done(Box<dyn Action>),
}
pub(super) struct ActionPayload<'a> {
    pub(super) name: &'a String,
    pub(super) account: &'a Weak<RefCell<Account>>,
    pub(super) book: &'a Book,
    pub(super) store: &'a Store,
}
pub(super) trait Action {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult;
}
