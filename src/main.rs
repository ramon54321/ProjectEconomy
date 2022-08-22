use economy::bank::*;

fn main() {
    let bank = FederalReserve::new("Washington");
    let account_a = bank.borrow_mut().open_account(bank.clone(), "Johnny");
    let account_b = bank.borrow_mut().open_account(bank.clone(), "Jill");

    bank.borrow_mut()
        .transfer(account_a.clone(), account_b.clone(), 500);

    println!("{:?}", account_a.borrow().bank.upgrade().unwrap().borrow());
}
