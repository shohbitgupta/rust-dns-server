use sha2::{Digest, Sha256};
use std::fmt::Debug;

use super::account::Account;
use super::helper::get_current_timestamp;
use super::transaction::{BlockTransaction, MerkleTree};

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
        merkle_tree.process_transactions(transactions.clone());

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

    // Apply the transactions in the block to the account state
    pub fn apply_transactions(&self, accounts: &mut Vec<Account>) -> bool {
        for tx in &self.transactions {
            if !tx.is_valid(accounts) {
                return false; // Transaction failed validation (insufficient funds)
            }
        }

        // Apply the transactions
        for tx in &self.transactions {
            match tx.execute(accounts) {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }

        true
    }
}
