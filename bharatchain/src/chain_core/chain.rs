use super::block::Block;

#[derive(Debug)]
pub struct BharatChain {
    pub chain: Vec<Block>,
}

impl BharatChain {
    // Create a new blockchain with a genesis block
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string(), vec![]);
        BharatChain {
            chain: vec![genesis_block],
        }
    }

    // Get the latest block in the chain
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // Add a new block to the blockchain
    pub fn add_block(&mut self, data: String) {
        print!("Adding block {} on chain....\n", data);
        let latest_block = self.get_latest_block();
        let new_block = Block::new(
            latest_block.index + 1,
            data,
            latest_block.hash.clone(),
            vec![],
        );
        self.chain.push(new_block);
    }
}
