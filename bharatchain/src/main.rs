mod chain_core;
use chain_core::chain::BharatChain;
use chain_core::transaction::{get_current_timestamp, BlockTransaction};

fn main() {
    // Set difficulty to 4 (requires 4 leading zeros in the hash)
    let mut blockchain = BharatChain::new(4);

    println!("Mining genesis block...");
    println!(
        "Genesis Block Hash: {}",
        blockchain.get_latest_block().block_hash
    );

    // Add some blocks with transactions
    blockchain.add_block(get_txns());
    blockchain.add_block(get_txns());

    // Verify the blockchain is valid
    if blockchain.is_valid() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain is invalid!");
    }

    blockchain.history();
}

fn get_txns() -> Vec<BlockTransaction> {
    // Create some transactions
    let tx1 = BlockTransaction {
        id: "tx1".to_string(),
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 50.0,
        timestamp: get_current_timestamp(),
    };

    let tx2 = BlockTransaction {
        id: "tx2".to_string(),
        sender: "Bob".to_string(),
        receiver: "Charlie".to_string(),
        amount: 30.0,
        timestamp: get_current_timestamp(),
    };

    let tx3 = BlockTransaction {
        id: "tx3".to_string(),
        sender: "Charlie".to_string(),
        receiver: "Dave".to_string(),
        amount: 20.0,
        timestamp: get_current_timestamp(),
    };

    // Create a block with these transactions
    return vec![tx1, tx2, tx3];
}
