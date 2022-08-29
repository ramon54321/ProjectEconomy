use std::{cell::RefCell, rc::Rc};

use crate::accounting::{account::Account, bank::Bank};

pub struct Loan {
    issuer: Rc<RefCell<dyn Bank>>,
    to: Rc<RefCell<Account>>,
    pub(crate) due: u64,
}
impl Loan {
    pub fn new(
        issuer: Rc<RefCell<dyn Bank>>,
        to: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            issuer,
            to,
            due: amount,
        }))
    }
}
