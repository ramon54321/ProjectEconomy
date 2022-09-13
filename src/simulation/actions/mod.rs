use super::{accounting::account::Account, book::Book, market::Market, store::Store};
use std::{cell::RefCell, rc::Weak};

pub(super) mod idle_action;

pub(super) enum ActionResult {
    InProgress,
    Done(Box<dyn Action>),
}
pub(super) struct ActionPayload<'a> {
    pub(super) name: &'a mut String,
    pub(super) account: &'a mut Weak<RefCell<Account>>,
    pub(super) book: &'a mut Book,
    pub(super) store: &'a mut Store,
    pub(super) market: &'a mut Market,
}
pub(super) trait Action {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult;
    fn get_name(&self) -> String;
}
