use super::{bank::Bank, loan::Loan, transaction::Transaction};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use uuid::Uuid;

pub struct Account {
    id: Uuid,
    name: String,
    bank: Weak<RefCell<Bank>>,
    loans: Vec<Weak<RefCell<Loan>>>,
    transactions: Vec<Weak<Transaction>>,
}
impl Account {
    pub(super) fn new(name: &str, bank: Weak<RefCell<Bank>>) -> Rc<RefCell<Account>> {
        Rc::new(RefCell::new(Account {
            id: Uuid::new_v4(),
            name: name.to_string(),
            bank,
            loans: Vec::new(),
            transactions: Vec::new(),
        }))
    }
    pub(super) fn add_loan(&mut self, loan: Weak<RefCell<Loan>>) {
        self.loans.push(loan);
    }
    pub(super) fn add_transaction(&mut self, transaction: Weak<Transaction>) {
        self.transactions.push(transaction);
    }
    pub(super) fn get_transactions(&self) -> &Vec<Weak<Transaction>> {
        &self.transactions
    }
    pub(crate) fn get_balance(&self) -> i64 {
        self.transactions.iter().fold(0, |acc, cur| {
            if let Some(transaction) = cur.upgrade() {
                if transaction
                    .from
                    .upgrade()
                    .is_some_and(|from| from.borrow().id == self.id)
                {
                    return acc - transaction.amount as i64;
                } else if transaction
                    .to
                    .upgrade()
                    .is_some_and(|to| to.borrow().id == self.id)
                {
                    return acc + transaction.amount as i64;
                }
            }
            acc
        })
    }
    pub(crate) fn get_name(&self) -> String {
        self.name.clone()
    }
    pub(crate) fn get_bank(&self) -> Weak<RefCell<Bank>> {
        self.bank.clone()
    }
}
