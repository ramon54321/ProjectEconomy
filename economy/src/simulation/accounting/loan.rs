use super::{account::Account, bank::Bank};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Loan {
    issuer: Weak<RefCell<Bank>>,
    to: Weak<RefCell<Account>>,
    due: u64,
}
impl Loan {
    pub fn new(
        issuer: Weak<RefCell<Bank>>,
        to: Weak<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            issuer,
            to,
            due: amount,
        }))
    }
    pub(super) fn get_due(&self) -> u64 {
        self.due
    }
}
