use super::{account::Account, loan::Loan, transaction::Transaction};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub struct Bank {
    weak_self: Weak<RefCell<Bank>>,
    name: String,
    accounts: Vec<Rc<RefCell<Account>>>,
    loans: Vec<Rc<RefCell<Loan>>>,
    transactions: Vec<Rc<Transaction>>,
}
impl Bank {
    pub fn new(name: &str) -> Rc<RefCell<Bank>> {
        Rc::new_cyclic(|bank_weak| {
            RefCell::new(Self {
                weak_self: bank_weak.clone(),
                name: name.to_string(),
                accounts: Vec::new(),
                loans: Vec::new(),
                transactions: Vec::new(),
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
    pub fn process_transaction(
        from: Weak<RefCell<Account>>,
        to: Weak<RefCell<Account>>,
        amount: u64,
    ) -> bool {
        let transaction = Transaction::new(from.clone(), to.clone(), amount);

        // Ensure validity of accounts
        let from_account = from.upgrade();
        let to_account = to.upgrade();
        if from_account.is_none() || to_account.is_none() {
            return false;
        }
        let from_account = from_account.unwrap();
        let to_account = to_account.unwrap();

        // Ensure validity of banks
        let from_bank = from_account.borrow_mut().get_bank().upgrade();
        let to_bank = to_account.borrow_mut().get_bank().upgrade();
        if from_bank.is_none() || to_bank.is_none() {
            return false;
        }
        let from_bank = from_bank.unwrap();
        let to_bank = to_bank.unwrap();

        // Add strong references to banks
        from_bank
            .borrow_mut()
            .transactions
            .push(transaction.clone());
        to_bank.borrow_mut().transactions.push(transaction.clone());

        // Add weak references to accounts
        from_account
            .borrow_mut()
            .add_transaction(Rc::downgrade(&transaction));
        to_account
            .borrow_mut()
            .add_transaction(Rc::downgrade(&transaction));

        // Return success
        true
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

    #[test]
    fn process_transaction() {
        let bank_a = Bank::new("Federal Reserve");
        let bank_b = Bank::new("Bank of America");
        let fed_account = bank_a.borrow_mut().open_account("FED");
        let boa_account = bank_b.borrow_mut().open_account("BOA");
        let was_transaction_success =
            Bank::process_transaction(fed_account.clone(), boa_account.clone(), 500);
        assert!(was_transaction_success);
        assert_eq!(bank_a.borrow().transactions.len(), 1);
        assert_eq!(bank_b.borrow().transactions.len(), 1);
        assert_eq!(fed_account.upgrade().unwrap().borrow().get_balance(), -500);
        assert_eq!(boa_account.upgrade().unwrap().borrow().get_balance(), 500);
        assert_eq!(
            fed_account
                .upgrade()
                .unwrap()
                .borrow()
                .get_transactions()
                .len(),
            1
        );
        assert_eq!(
            boa_account
                .upgrade()
                .unwrap()
                .borrow()
                .get_transactions()
                .len(),
            1
        );
    }
}
