#[derive(Debug)]
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: u32, holder_name: String, initial_deposit: f64) -> Self {
        Self {
            account_number,
            holder_name,
            balance: initial_deposit,
        }
    }

    fn check_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if self.balance > amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    fn close_account(self) -> f64 {
        self.balance
    }
}

fn main() {
    let mut acc = BankAccount::new(101, String::from("Amit"), 500.0);

    println!("Initial Balance: ${}", acc.check_balance());

    acc.deposit(250.0);
    println!("After Deposit: ${}", acc.check_balance());

    let ok = acc.withdraw(100.0);
    println!(
        "Withdraw 100 successful? {} | Balance: ${}",
        ok,
        acc.check_balance()
    );

    let failed = acc.withdraw(1000.0);
    println!(
        "Withdraw 1000 successful? {} | Balance: ${}",
        failed,
        acc.check_balance()
    );

    let remaining_cash = acc.close_account();
    println!("Account closed! Handed back: ${}", remaining_cash);

    // println!("Balance after close: {}", acc.check_balance());
}
