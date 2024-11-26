use super::account::Account;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender_address: String,
    pub receiver_address: String,
    pub amount: f64,
    pub id: String,
}

#[allow(dead_code)]
impl Transaction {
    // initialize transaction
    pub fn new(sender: &String, receiver: &String, amount: f64) -> Self {
        let id = Transaction::get_txn_hash(sender, receiver);
        Transaction {
            sender_address: sender.clone(),
            receiver_address: receiver.clone(),
            amount,
            id,
        }
    }

    // Calculate the hash for a block
    fn get_txn_hash(sender: &String, receiver: &String) -> String {
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut hasher = Sha256::new();

        hasher.update(time_stamp.to_string());
        hasher.update(sender);
        hasher.update(receiver);

        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Execute the transaction: Debit from sender and credit to receiver.
    pub fn execute(&self, accounts: &mut Vec<Account>) -> Result<(), String> {
        let mut mutable_accounts_list = accounts.iter_mut(); // iter_mut() for mutable reference

        let sender_account = mutable_accounts_list.find(|a| a.address == self.sender_address);
        let receiver_account = mutable_accounts_list.find(|a| a.address == self.receiver_address);

        match (sender_account, receiver_account) {
            (Some(sender), Some(receiver)) => {
                sender.debit(self.amount)?;
                receiver.credit(self.amount);
                Ok(())
            }
            _ => Err("Sender or Receiver account not found".to_string()),
        }
    }
}
