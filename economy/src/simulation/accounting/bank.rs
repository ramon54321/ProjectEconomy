use super::{account::Account, loan::Loan};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Bank {
    weak_self: Weak<RefCell<Bank>>,
    name: String,
    accounts: Vec<Rc<RefCell<Account>>>,
    loans: Vec<Rc<RefCell<Loan>>>,
}
impl Bank {
    pub fn new(name: &str) -> Rc<RefCell<Bank>> {
        Rc::new_cyclic(|bank_weak| {
            RefCell::new(Self {
                weak_self: bank_weak.clone(),
                name: name.to_string(),
                accounts: Vec::new(),
                loans: Vec::new(),
            })
        })
    }
    pub fn open_account(&mut self, name: &str) -> Weak<RefCell<Account>> {
        let account = Account::new(name, self.weak_self.clone());
        self.accounts.push(account.clone());
        Rc::downgrade(&account)
    }
    pub fn issue_loan(
        &mut self,
        account: Weak<RefCell<Account>>,
        amount: u64,
    ) -> Option<Weak<RefCell<Loan>>> {
        let account = account.upgrade()?;
        let loan = Loan::new(self.weak_self.clone(), Rc::downgrade(&account), amount);
        let weak_loan = Rc::downgrade(&loan);
        self.loans.push(loan);
        account.borrow_mut().add_loan(weak_loan.clone());
        Some(weak_loan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_account() {
        let bank = Bank::new("Federal Reserve");
        let account = bank.borrow_mut().open_account("Jeff");
        assert_eq!(
            account.upgrade().unwrap().borrow().get_name(),
            "Jeff".to_string()
        );
        assert_eq!(bank.borrow().accounts.len(), 1);
    }

    #[test]
    fn issue_loan() {
        let bank = Bank::new("Federal Reserve");
        let account = bank.borrow_mut().open_account("Jeff");
        let loan = bank.borrow_mut().issue_loan(account, 500);
        assert!(loan.is_some());
        assert_eq!(loan.unwrap().upgrade().unwrap().borrow().get_due(), 500);
    }
}
