use super::transaction::{BlockTransaction, DataBlock};

#[derive(Debug)]
pub struct BharatChain {
    pub chain: Vec<DataBlock>,
    pub difficulty: usize,
}

impl BharatChain {
    // Create a new blockchain with a genesis block
    pub fn new(difficulty: usize) -> Self {
        let genesis_block_prvious_hash =
            String::from("27d9e52ddb66a5e2d1adeac33afcc9a1cf64847064760fa49cdf4eeb110c4953");

        let genesis_block: DataBlock = DataBlock::new(0, genesis_block_prvious_hash, vec![]);

        BharatChain {
            chain: vec![genesis_block],
            difficulty,
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
        let mut block_to_mine = new_block;
        block_to_mine.mine_block(self.difficulty);
        self.chain.push(block_to_mine);
    }

    // Check if the blockchain is valid (basic check for now)
    pub fn is_valid(&self) -> bool {
        let length = self.chain.len();
        for i in 1..length {
            let previous_block = &self.chain[i - 1];
            let current_block = &self.chain[i];

            // Check that the previous block's hash matches the current block's "previous_hash"
            if current_block.previous_hash != previous_block.block_hash {
                return false;
            }

            // Check that the current block's hash matches the calculated hash
            if current_block.block_hash != current_block.calculate_hash() {
                return false;
            }
        }
        true
    }

    pub fn history(&mut self) {
        print!("\n Blockchain History......:\n");
        for (index, block) in self.chain.iter().enumerate() {
            println!("Block #{}:", index);
            println!("- Block Number: {}", block.block_number);
            println!("- Previous Hash: {}", block.previous_hash);
            println!("- Merkle Root: {}", block.merkle_root);
            println!("- Block timestamp: {} \n", block.timestamp);
            println!("- Block hash: {} \n", block.block_hash);
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
