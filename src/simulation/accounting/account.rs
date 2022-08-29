use super::{bank::Bank, loan::Loan};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Account {
    name: String,
    bank: Weak<RefCell<Bank>>,
    loans: Vec<Weak<RefCell<Loan>>>,
}
impl Account {
    pub(super) fn new(name: &str, bank: Weak<RefCell<Bank>>) -> Rc<RefCell<Account>> {
        Rc::new(RefCell::new(Account {
            name: name.to_string(),
            bank,
            loans: Vec::new(),
        }))
    }
    pub(super) fn add_loan(&mut self, loan: Weak<RefCell<Loan>>) {
        self.loans.push(loan);
    }
    pub(crate) fn get_balance(&self) -> i64 {
        0
    }
    pub(crate) fn get_name(&self) -> String {
        self.name.clone()
    }
}
