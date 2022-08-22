use crate::account::Account;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub struct Transaction {
    pub(crate) from: Rc<RefCell<Account>>,
    pub(crate) to: Rc<RefCell<Account>>,
    pub(crate) amount: u64,
}
impl Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction of {} from {} to {}",
            self.amount,
            self.from.borrow().name,
            self.to.borrow().name,
        )
    }
}
