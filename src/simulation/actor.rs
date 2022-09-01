use super::{
    accounting::{account::Account, bank::Bank},
    actions::{idle_action::IdleAction, Action, ActionPayload, ActionResult},
    book::Book,
    market::Market,
    store::Store,
};
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

pub struct Actor {
    name: String,
    account: Weak<RefCell<Account>>,
    book: Book,
    store: Store,
    action: Box<dyn Action>,
}
impl Actor {
    pub(super) fn new(name: &str, bank: Rc<RefCell<Bank>>) -> Self {
        let account = bank.borrow_mut().open_account(name);
        Self {
            name: name.to_string(),
            account,
            book: Book::new(),
            store: Store::new(),
            action: Box::new(IdleAction::new()),
        }
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
            store: &mut self.store,
            market,
        });
        match action_result {
            ActionResult::InProgress => (),
            ActionResult::Done(next_action) => self.action = next_action,
        };
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
