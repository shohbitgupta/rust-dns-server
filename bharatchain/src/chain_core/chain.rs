use super::transaction::{BlockTransaction, DataBlock};

#[derive(Debug)]
pub struct BharatChain {
    pub chain: Vec<DataBlock>,
}

impl BharatChain {
    // Create a new blockchain with a genesis block
    pub fn new() -> Self {
        let genesis_block = DataBlock::new(0, "Genesis Block".to_string(), vec![]);
        BharatChain {
            chain: vec![genesis_block],
        }
    }

    // Get the latest block in the chain
    pub fn get_latest_block(&self) -> &DataBlock {
        self.chain.last().unwrap()
    }

    // Add a new block to the blockchain
    pub fn add_block(&mut self, txns: Vec<BlockTransaction>) {
        print!("\n Adding block on chain....\n");
        let latest_block = self.get_latest_block();
        let new_block = DataBlock::new(
            latest_block.block_number + 1,
            latest_block.merkle_root.clone(),
            txns,
        );
        self.chain.push(new_block);
    }

    pub fn history(&mut self) {
        print!("\n Blockchain History......:\n");
        for (index, block) in self.chain.iter().enumerate() {
            println!("Block #{}:", index);
            println!("- Block Number: {}", block.block_number);
            println!("- Previous Hash: {}", block.previous_hash);
            println!("- Merkle Root: {}", block.merkle_root);
            println!("- Block timestamp: {} \n", block.timestamp);
            println!("- Transactions:\n");
            for tx in &block.transactions {
                println!("  - ID: {}", tx.id);
                println!("  - Sender: {}", tx.sender);
                println!("  - Receiver: {}", tx.receiver);
                println!("  - Amount: {}", tx.amount);
                println!("  - Timestamp: {}", tx.timestamp);
            }
            println!("---\n");
        }
    }
}
