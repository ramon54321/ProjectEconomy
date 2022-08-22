use crate::{bank::Bank, transaction::Transaction};
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

pub struct Account {
    pub name: String,
    pub transactions: Vec<Rc<RefCell<Transaction>>>,
    pub bank: Weak<RefCell<dyn Bank>>,
}
impl Account {
    pub(crate) fn new(name: &str, bank: Rc<RefCell<dyn Bank>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Account {
            name: name.to_string(),
            transactions: Vec::new(),
            bank: Rc::downgrade(&bank),
        }))
    }
    pub(crate) fn add_transaction(&mut self, transaction: Rc<RefCell<Transaction>>) {
        self.transactions.push(transaction);
    }
    pub(crate) fn get_balance(&self) -> i64 {
        self.transactions.iter().fold(0, |acc, transaction| {
            if transaction.borrow().from.borrow().name == self.name {
                acc - transaction.borrow().amount as i64
            } else {
                acc + transaction.borrow().amount as i64
            }
        })
    }
}
impl Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Account for {} with {} transactions and a balance of {}",
            self.name,
            self.transactions.len(),
            self.get_balance()
        )
    }
}
