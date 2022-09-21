use super::{
    accounting::{account::Account, bank::Bank},
    actions::{work_action::WorkAction, Action, ActionPayload, ActionResult},
    book::Book,
    market::{listing::Listing, Market},
    store::Store,
};
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};
use uuid::Uuid;

pub struct Actor {
    pub(super) id: Uuid,
    name: String,
    account: Weak<RefCell<Account>>,
    book: Book,
    submitted_listings: Vec<Weak<Listing>>,
    store_actual: Store,
    store_target: Store,
    action: Box<dyn Action>,
}
impl Actor {
    pub(super) fn new(name: &str, bank: Rc<RefCell<Bank>>) -> Rc<RefCell<Self>> {
        let account = bank.borrow_mut().open_account(name);
        Rc::new(RefCell::new(Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            account,
            book: Book::new(),
            submitted_listings: Vec::new(),
            store_actual: Store::new(),
            store_target: Store::new(),
            action: Box::new(WorkAction::new()),
        }))
    }
    ///
    /// Dispatches the current state of the actor to the current action held by the actor. The
    /// action will either tick and return a 'InProgress' state or will return a 'Done' state
    /// containing the next action. This tick method is responsible for replacing the old action
    /// with the newly returned action in preparation for the next call to this tick method.
    ///
    pub(super) fn tick(&mut self, market: &mut Market) {
        let action_result = self.action.tick(ActionPayload {
            name: &mut self.name,
            account: &mut self.account,
            book: &mut self.book,
            submitted_listings: &mut self.submitted_listings,
            store_actual: &mut self.store_actual,
            store_target: &mut self.store_target,
            market,
        });
        match action_result {
            ActionResult::InProgress => (),
            ActionResult::Done(next_action) => self.action = next_action,
        };
    }
    pub(super) fn get_account(&self) -> Weak<RefCell<Account>> {
        self.account.clone()
    }
}

impl Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} - {:?}",
            self.name,
            self.account.upgrade().unwrap().borrow().get_balance(),
            self.book,
        )
    }
}
