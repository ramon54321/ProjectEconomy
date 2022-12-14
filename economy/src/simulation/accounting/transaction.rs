use super::account::Account;
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

pub struct Transaction {
    pub(super) from: Weak<RefCell<Account>>,
    pub(super) to: Weak<RefCell<Account>>,
    pub(super) amount: u64,
}
impl Transaction {
    pub(super) fn new(
        from: Weak<RefCell<Account>>,
        to: Weak<RefCell<Account>>,
        amount: u64,
    ) -> Rc<Self> {
        Rc::new(Transaction { from, to, amount })
    }
}
impl Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction of {} from {} to {}",
            self.amount,
            self.from.upgrade().unwrap().borrow().get_name(),
            self.to.upgrade().unwrap().borrow().get_name(),
        )
    }
}
