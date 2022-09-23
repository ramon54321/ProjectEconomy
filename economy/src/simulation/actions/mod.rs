use super::{
    accounting::account::Account,
    actor::Actor,
    book::Book,
    logbook::LogBook,
    market::{listing::Listing, Market},
    store::Store,
    task::Task,
};
use std::{cell::RefCell, rc::Weak};

pub(super) mod work_action;

pub(super) enum ActionResult {
    InProgress,
    Done(Box<dyn Action>),
}
pub(super) struct ActionPayload<'a> {
    pub(super) actor_weak: Weak<RefCell<Actor>>,
    pub(super) name: &'a mut String,
    pub(super) log: &'a mut LogBook,
    pub(super) account: &'a mut Weak<RefCell<Account>>,
    pub(super) book: &'a mut Book,
    pub(super) submitted_listings: &'a mut Vec<Weak<Listing>>,
    pub(super) store_actual: &'a mut Store,
    pub(super) store_target: &'a mut Store,
    pub(super) market: &'a mut Market,
    pub(super) task: &'a Option<Task>,
}
pub(super) trait Action {
    fn tick(&mut self, payload: ActionPayload) -> ActionResult;
    fn get_name(&self) -> String;
}
