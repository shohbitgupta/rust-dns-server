// use serde::{Deserialize, Serialize};
// use sha2::{Digest, Sha256};
// use std::time::{SystemTime, UNIX_EPOCH};

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Block {
//     pub index: u32,
//     pub timestamp: u64,
//     pub transactions: Vec<Transaction>,
//     pub prev_block_hash: String,
//     pub hash: String,
//     pub nonce: u128,
//     pub data: String,
// }

// impl Block {
//     pub fn new(
//         index: u32,
//         data: String,
//         previous_hash: String,
//         transactions: Vec<Transaction>,
//     ) -> Block {
//         let timestamp = Block::get_current_timestamp();
//         let nonce = 0; // Initial nonce
//         let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash, nonce);
//         Block {
//             index: index,
//             timestamp: timestamp,
//             data: data,
//             transactions: transactions,
//             prev_block_hash: previous_hash,
//             hash: hash,
//             nonce,
//         }
//     }

//     pub fn get_current_timestamp() -> u64 {
//         SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .expect("Time went backwards")
//             .as_secs()
//     }

//     // Calculate the hash for a block
//     pub fn calculate_hash(
//         index: u32,
//         timestamp: u64,
//         data: &str,
//         previous_hash: &str,
//         difficulty: u128,
//     ) -> String {
//         let mut hasher = Sha256::new();
//         hasher.update(index.to_string());
//         hasher.update(timestamp.to_string());
//         hasher.update(data);
//         hasher.update(previous_hash);
//         hasher.update(difficulty.to_string());
//         let result = hasher.finalize();
//         format!("{:x}", result)
//     }
// }
