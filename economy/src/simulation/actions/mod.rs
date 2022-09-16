use super::{
    accounting::account::Account,
    book::Book,
    market::{listing::Listing, Market},
    store::Store,
};
use std::{cell::RefCell, rc::Weak};

pub(super) mod work_action;

pub(super) enum ActionResult {
    InProgress,
    Done(Box<dyn Action>),
}
pub(super) struct ActionPayload<'a> {
    pub(super) name: &'a mut String,
    pub(super) account: &'a mut Weak<RefCell<Account>>,
    pub(super) book: &'a mut Book,
    pub(super) submitted_listings: &'a mut Vec<Weak<Listing>>,
    pub(super) store_actual: &'a mut Store,
    pub(super) store_target: &'a mut Store,
    pub(super) market: &'a mut Market,
}
pub(super) trait Action {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult;
    fn get_name(&self) -> String;
}
