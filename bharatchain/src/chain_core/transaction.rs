/*use super::account::Account;
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
}*/

use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct BlockTransaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
}

impl BlockTransaction {
    // Helper function to create a transaction hash based on its content
    pub fn compute_hash(&self) -> String {
        let serialized = serde_json::to_string(&self).unwrap();
        let hash = Keccak256::digest(serialized.as_bytes());
        format!("{:x}", hash)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub block_number: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub transactions: Vec<BlockTransaction>,
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
    pub fn insert_transactions(&mut self, transactions: Vec<BlockTransaction>) {
        let transaction_hashes: Vec<String> = transactions
            .into_iter()
            .map(|tx| tx.compute_hash())
            .collect();
        self.root = self.build_merkle_tree(transaction_hashes);
    }

    // Build the Merkle tree from transaction hashes and compute the root
    fn build_merkle_tree(&mut self, transaction_hashes: Vec<String>) -> String {
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
        let hash = Keccak256::digest(combined.as_bytes());
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

#[derive(Debug)]

pub struct DataBlock {
    pub block_number: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub transactions: Vec<BlockTransaction>,
    pub timestamp: u64,
}

impl DataBlock {
    pub fn new(
        block_number: u64,
        previous_hash: String,
        transactions: Vec<BlockTransaction>,
    ) -> Self {
        // Create the Merkle tree and calculate the Merkle root
        let mut merkle_tree = MerkleTree::new();
        merkle_tree.insert_transactions(transactions.clone());

        DataBlock {
            block_number,
            previous_hash,
            merkle_root: merkle_tree.get_merkle_root(),
            transactions,
            timestamp: get_current_timestamp(),
        }
    }
}

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
