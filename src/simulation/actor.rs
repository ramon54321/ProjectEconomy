use super::{
    accounting::{account::Account, bank::Bank},
    book::Book,
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
}
impl Actor {
    pub(super) fn new(name: &str, bank: Rc<RefCell<Bank>>) -> Self {
        let account = bank.borrow_mut().open_account(name);
        Self {
            name: name.to_string(),
            account,
            book: Book::new(),
        }
    }
    pub(super) fn tick(&mut self) {
        println!("Ticking {:?}", self);
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
