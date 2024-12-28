use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fmt::Debug;

use super::account::Account;
use super::helper;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct BlockTransaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
}

impl BlockTransaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        let time_stamp = helper::get_current_timestamp();
        BlockTransaction {
            sender,
            receiver,
            amount,
            id: time_stamp.to_string(),
            timestamp: time_stamp,
        }
    }

    // Helper function to create a transaction hash based on its content
    pub fn compute_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}",
            self.id, self.timestamp, self.receiver, self.amount, self.sender
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Validate that the sender has enough funds
    pub fn is_valid(&self, accounts: &mut Vec<Account>) -> bool {
        // It sayes account creation txn received.
        if self.sender.to_lowercase() == "system" {
            return true;
        }

        let sender_account = accounts.iter_mut().find(|a| a.address == self.sender);
        match sender_account {
            Some(account) => account.balance >= self.amount,
            None => false,
        }
    }

    pub fn execute(&self, accounts: &mut Vec<Account>) -> Result<(), String> {
        print!(
            "\n ------- Executing transaction from {} -------- \n ",
            self.sender
        );
        if self.sender.to_lowercase() == "system" {
            // Special case: Create a new account
            let acc = accounts.iter().find(|a| a.address == self.receiver);
            match acc {
                Some(_a) => {
                    let new_acc: Option<Account> =
                        Account::from_secret_key(&self.receiver, self.amount).ok();
                    match new_acc {
                        Some(ac) => {
                            accounts.push(ac);
                            println!(
                                "Account created | at: address = {}, initial balance = {}",
                                self.receiver, self.amount
                            );
                            return Ok(());
                        }
                        None => (),
                    }
                }
                None => {
                    println!("Transaction ABORT | Account creation failed with Rreason:- Account mismatch");
                    ()
                }
            }
        }

        let mut mutable_accounts_list = accounts.iter_mut(); // iter_mut() for mutable reference

        let sender_account = mutable_accounts_list.find(|a| a.address == self.sender);
        let receiver_account = mutable_accounts_list.find(|a| a.address == self.receiver);

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

// Node enum for Merkle tree with transaction data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Leaf { transaction_hash: String },
    Branch { children: Vec<Option<String>> },
}

// Merkle tree structure for storing transaction hashes
#[derive(Debug)]
pub struct MerkleTree {
    root: String,                 // Merkle root hash
    nodes: HashMap<String, Node>, // Hashmap to store nodes by their hash
}

impl MerkleTree {
    // Create a new Merkle tree (empty)
    pub fn new() -> Self {
        MerkleTree {
            root: String::new(),
            nodes: HashMap::new(),
        }
    }

    // Insert transactions into the Merkle tree
    pub fn process_transactions(&mut self, transactions: Vec<BlockTransaction>) {
        let transaction_hashes: Vec<String> = transactions
            .into_iter()
            .map(|tx| tx.compute_hash())
            .collect();
        self.root = self.build_merkle_tree(transaction_hashes);
    }

    // Build the Merkle tree from transaction hashes and compute the root
    pub fn build_merkle_tree(&mut self, transaction_hashes: Vec<String>) -> String {
        let mut current_level = transaction_hashes;

        // Loop until there is only one node left, the root of the tree
        while current_level.len() > 1 {
            let mut next_level = vec![];

            for chunk in current_level.chunks(2) {
                let left = chunk[0].clone();
                let right = chunk.get(1).cloned().unwrap_or_else(|| left.clone());

                let combined_hash = self.hash_pair(left, right);
                next_level.push(combined_hash);
            }

            current_level = next_level;
        }

        // The remaining item is the root of the Merkle tree
        if current_level.len() > 0 {
            return current_level[0].clone();
        }

        return "0x00000000".to_string();
    }

    // Hash two transaction hashes together to form the parent node
    fn hash_pair(&mut self, left: String, right: String) -> String {
        let combined = format!("{}{}", left, right);
        let hash = Sha256::digest(combined.as_bytes());
        let hash_str = format!("{:x}", hash);
        self.nodes.insert(
            hash_str.clone(),
            Node::Branch {
                children: vec![Some(left), Some(right)],
            },
        );
        hash_str
    }

    // Get the Merkle root
    pub fn get_merkle_root(&self) -> String {
        self.root.clone()
    }
}
