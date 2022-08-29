use crate::accounting::account::Account;
use crate::accounting::loan::Loan;
use crate::accounting::transaction::Transaction;
use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use super::transaction;

pub trait Bank: Debug {
    fn get_treasury_account(&self) -> Rc<RefCell<Account>>;
    fn transfer(
        &mut self,
        a: Rc<RefCell<Account>>,
        b: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Transaction>>;
    fn open_account(&mut self, bank: Rc<RefCell<dyn Bank>>, name: &str) -> Rc<RefCell<Account>>;
    fn issue_loan(
        &mut self,
        bank: Rc<RefCell<dyn Bank>>,
        account: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Loan>>;
}

pub struct FederalReserve {
    name: String,
    treasury: Rc<RefCell<Account>>,
    accounts: Vec<Rc<RefCell<Account>>>,
    transactions: Vec<Rc<RefCell<Transaction>>>,
}
impl FederalReserve {
    pub fn new(name: &str) -> Rc<RefCell<Self>> {
        let bank = Rc::new_cyclic(|bank| {
            RefCell::new(FederalReserve {
                name: name.to_string(),
                treasury: Rc::new(RefCell::new(Account::new("Treasury", bank.clone()))),
                accounts: Vec::new(),
                transactions: Vec::new(),
            })
        });
    }
}
impl Bank for FederalReserve {
    fn get_treasury_account(&self) -> Rc<RefCell<Account>> {}
    fn transfer(
        &mut self,
        a: Rc<RefCell<Account>>,
        b: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Transaction>> {
        let transaction = Transaction::new(a.clone(), b.clone(), amount);
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
    fn issue_loan(
        &mut self,
        bank: Rc<RefCell<dyn Bank>>,
        account: Rc<RefCell<Account>>,
        amount: u64,
    ) -> Rc<RefCell<Loan>> {
        let loan = Loan::new(bank, account.clone(), amount);
        account.borrow_mut().add_loan(loan.clone());
        loan
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
