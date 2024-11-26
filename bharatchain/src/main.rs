mod chain_core;
use chain_core::chain::BharatChain;

fn main() {
    let mut blockchain = BharatChain::new();

    // Add some blocks to the chain
    blockchain.add_block("First block".to_string());
    blockchain.add_block("Second block".to_string());
    blockchain.add_block("Third block".to_string());

    // Print the blockchain
    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }
}
