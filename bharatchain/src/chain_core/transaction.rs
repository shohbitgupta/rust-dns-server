use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt::Debug;
use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use super::account::Account;

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
        let block_data = format!(
            "{}{}{}{}{}",
            self.id, self.timestamp, self.receiver, self.amount, self.sender
        );
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn execute(&self, accounts: &mut Vec<Account>) -> Result<(), String> {
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

#[derive(Debug)]

pub struct DataBlock {
    pub block_number: u64,
    pub previous_hash: String,
    pub merkle_root: String,
    pub block_hash: String,
    pub transactions: Vec<BlockTransaction>,
    pub timestamp: u64,
    pub nounce: u64,
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

        let mut block = DataBlock {
            block_number,
            previous_hash,
            merkle_root: merkle_tree.get_merkle_root(),
            transactions,
            timestamp: get_current_timestamp(),
            nounce: 0,
            block_hash: String::new(),
        };

        block.block_hash = block.calculate_hash();
        block
    }

    // Calculate the hash of the block (with nonce and Merkle root)
    pub fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}{}", // Index, Timestamp, Transactions (Merkle root), Previous hash, Nonce
            self.block_number,
            self.timestamp,
            self.merkle_root,
            self.previous_hash,
            self.nounce,
            self.timestamp, // Adding a timestamp to make it unique
        );

        let mut hasher = Sha256::new();
        hasher.update(block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Perform proof-of-work to find a valid hash
    pub fn mine_block(&mut self, difficulty: usize) {
        let target = vec!['0'; difficulty]; // "difficulty" number of leading zeros
        while &self.block_hash[..difficulty] != target.iter().collect::<String>() {
            self.nounce += 1;
            self.block_hash = self.calculate_hash();
        }
        println!(
            "Block mined! Nonce: {}, Hash: {}",
            self.nounce, self.block_hash
        );
    }
}

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
