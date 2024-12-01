mod chain_core;
use chain_core::chain::BharatChain;
use chain_core::transaction::{get_current_timestamp, BlockTransaction};

fn main() {
    let mut blockchain = BharatChain::new();

    // // Add some blocks to the chain
    // blockchain.add_block("First block".to_string());
    // blockchain.add_block("Second block".to_string());
    // blockchain.add_block("Third block".to_string());

    // // Print the blockchain
    // for block in blockchain.chain.iter() {
    //     println!("{:?}", block);
    // }

    // let mtree = MerkelTree::new();
    // MerkelNode::new(mtree);

    let mut i = 0;
    while i < 10 {
        blockchain.add_block(get_txns());
        i += 1;
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
