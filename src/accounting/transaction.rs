use crate::accounting::account::Account;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub struct Transaction {
    pub(crate) from: Rc<RefCell<Account>>,
    pub(crate) to: Rc<RefCell<Account>>,
    pub(crate) amount: u64,
}
impl Transaction {
    pub(crate) fn new(
        from: Rc<RefCell<Account>>,
        to: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Transaction { from, to, amount }))
    }
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
