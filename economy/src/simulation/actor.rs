use super::{
    accounting::{account::Account, bank::Bank},
    actions::{work_action::WorkAction, Action, ActionPayload, ActionResult},
    book::Book,
    logbook::LogBook,
    market::{listing::Listing, Market},
    store::Store,
    task::Task,
};
use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    rc::{Rc, Weak},
};
use uuid::Uuid;

pub struct Actor {
    weak_self: Weak<RefCell<Actor>>,
    pub(super) id: Uuid,
    name: String,
    log: LogBook,
    account: Weak<RefCell<Account>>,
    book: Book,
    submitted_listings: Vec<Weak<Listing>>,
    store_actual: Store,
    store_target: Store,
    action: Box<dyn Action>,
    task: Option<Task>,
}
impl Actor {
    pub(super) fn new(
        name: &str,
        bank: Rc<RefCell<Bank>>,
        task: Option<Task>,
    ) -> Rc<RefCell<Self>> {
        let account = bank.borrow_mut().open_account(name);
        Rc::new_cyclic(|weak_self| {
            RefCell::new(Self {
                weak_self: weak_self.clone(),
                id: Uuid::new_v4(),
                name: name.to_string(),
                log: LogBook::default(),
                account,
                book: Book::new(),
                submitted_listings: Vec::new(),
                store_actual: Store::new(),
                store_target: Store::new(),
                action: Box::new(WorkAction::new()),
                task,
            })
        })
    }
    ///
    /// Dispatches the current state of the actor to the current action held by the actor. The
    /// action will either tick and return a 'InProgress' state or will return a 'Done' state
    /// containing the next action. This tick method is responsible for replacing the old action
    /// with the newly returned action in preparation for the next call to this tick method.
    ///
    pub(super) fn tick(&mut self, market: &mut Market) {
        // Clean up listings
        self.submitted_listings
            .retain(|listing| listing.upgrade().is_some());

        let action_result = self.action.tick(ActionPayload {
            actor_weak: self.weak_self.clone(),
            name: &mut self.name,
            log: &mut self.log,
            account: &mut self.account,
            book: &mut self.book,
            submitted_listings: &mut self.submitted_listings,
            store_actual: &mut self.store_actual,
            store_target: &mut self.store_target,
            market,
            task: &self.task,
        });
        match action_result {
            ActionResult::InProgress => (),
            ActionResult::Done(next_action) => self.action = next_action,
        };
    }
    pub(super) fn get_account(&self) -> Weak<RefCell<Account>> {
        self.account.clone()
    }
    pub(super) fn get_name(&self) -> String {
        self.name.clone()
    }
    pub(super) fn get_log(&self) -> Vec<String> {
        self.log.get_entries()
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
