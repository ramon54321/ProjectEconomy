use crate::accounting::account::Account;
use crate::accounting::bank::{Bank, FederalReserve};
use crate::book::Book;
use std::fmt::Debug;
use std::{cell::RefCell, rc::Rc};

pub struct Actor {
    name: String,
    account: Rc<RefCell<Account>>,
    book: Book,
}
impl Actor {
    pub fn new(name: &str, bank: Rc<RefCell<FederalReserve>>) -> Self {
        let account = bank.borrow_mut().open_account(bank.clone(), name);
        bank.borrow_mut()
            .issue_loan(bank.clone(), account.clone(), 500);
        Self {
            name: name.to_string(),
            account,
            book: Book::new(),
        }
    }
    pub fn tick(&mut self) {
        println!("Ticking {:?}", self);
    }
}

impl Debug for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - ${} - {:?}",
            self.name,
            self.account.borrow().get_balance(),
            self.book,
        )
    }
}
