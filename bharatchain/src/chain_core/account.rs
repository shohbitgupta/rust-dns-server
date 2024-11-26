#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Account {
    pub address: String,
    pub balance: f64,
}

#[allow(dead_code)]
impl Account {
    // Constructor to create a new account with a given address and initial balance.
    pub fn new(address: &str, balance: f64) -> Self {
        Account {
            address: address.to_string(),
            balance,
        }
    }

    // Method to debit the account (subtract balance).
    pub fn debit(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    // Method to credit the account (add balance).
    pub fn credit(&mut self, amount: f64) {
        self.balance += amount;
    }
}
