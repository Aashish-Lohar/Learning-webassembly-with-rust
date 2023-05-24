struct BankAccount {
    balance: i32,
    verified: bool,
}

// &borrowing variable takes ownership of variable to read the data
fn print_balance(account: &BankAccount) {
    println!("{:?}", account.balance);
}
fn print_verified(account: &BankAccount) {
    println!("{:?}", account.verified);
}

// Result data type
fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    return match account.verified {
        true => Ok(true),
        false => Err(false),
    };
}

fn main() {
    let my_account = BankAccount {
        balance: 20,
        verified: false,
    };
    let verification_status = is_verified(&my_account).expect("unable to unwrap result");

    // here & allows the function to borrow the variable values
    print_balance(&my_account);
    print_verified(&my_account);
    println!("{:?}", verification_status);
}
