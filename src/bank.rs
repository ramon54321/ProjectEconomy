use crate::account::Account;
use crate::transaction::Transaction;
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait Bank: Debug {
    fn transfer(
        &mut self,
        a: Rc<RefCell<Account>>,
        b: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Transaction>>;
    fn open_account(&mut self, bank: Rc<RefCell<dyn Bank>>, name: &str) -> Rc<RefCell<Account>>;
}

pub struct FederalReserve {
    name: String,
    accounts: Vec<Rc<RefCell<Account>>>,
    transactions: Vec<Rc<RefCell<Transaction>>>,
}
impl FederalReserve {
    pub fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(FederalReserve {
            name: name.to_string(),
            accounts: Vec::new(),
            transactions: Vec::new(),
        }))
    }
}
impl Bank for FederalReserve {
    fn transfer(
        &mut self,
        a: Rc<RefCell<Account>>,
        b: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Transaction>> {
        let transaction = Rc::new(RefCell::new(Transaction {
            from: a.clone(),
            to: b.clone(),
            amount,
        }));
        a.borrow_mut().add_transaction(transaction.clone());
        b.borrow_mut().add_transaction(transaction.clone());
        self.transactions.push(transaction.clone());
        transaction
    }
    fn open_account(&mut self, bank: Rc<RefCell<dyn Bank>>, name: &str) -> Rc<RefCell<Account>> {
        let account = Account::new(name, bank);
        self.accounts.push(account.clone());
        account
    }
}
impl Debug for FederalReserve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Federal Reserve:\n\tName: {}\n\tAccounts: {:?}\n\tTransactions: {:?}",
            self.name,
            self.accounts.iter().map(|t| t.borrow()).collect::<Vec<_>>(),
            self.transactions
                .iter()
                .map(|t| t.borrow())
                .collect::<Vec<_>>()
        )
    }
}
