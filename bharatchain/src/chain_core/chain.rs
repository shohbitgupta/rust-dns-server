use super::account::Account;
use super::block::DataBlock;
use super::transaction::{BlockTransaction, MerkleTree};
use rand::rngs::OsRng;
use secp256k1::SecretKey;

#[derive(Debug)]
pub struct BharatChain {
    pub chain: Vec<DataBlock>,
    pub difficulty: usize,
    pub accounts: Vec<Account>, // Track the state of all accounts
}

impl BharatChain {
    // Create a new blockchain with a genesis block
    pub fn new(difficulty: usize) -> Self {
        let genesis_block_prvious_hash =
            String::from("27d9e52ddb66a5e2d1adeac33afcc9a1cf64847064760fa49cdf4eeb110c4953");

        let genesis_block: DataBlock = DataBlock::new(0, genesis_block_prvious_hash, vec![]);
        let key1 = SecretKey::from_keypair(keypair);
        let alice = Account::from_secret_key(&, 1000.00);
        let bob = Account::from_secret_key(&hex::encode("Bob"), 500.00);

        match alice {
            Ok(_) => println!("Alice:"),
            Err(e) => println!("Error : {}", e),
        }

        match bob {
            Ok(_) => println!("Alice:"),
            Err(e) => println!("Error : {}", e),
        }

        // let accounts: Vec<Result<Account, String>> = vec![alice, bob];

        // let acc_list: Vec<Account> = accounts
        //     .into_iter()
        //     .filter_map(|res| res.ok()) // Extract only the Ok values, discarding Err
        //     .collect();

        // print!("Initializing blockchain... {} \n", acc_list.len());

        BharatChain {
            chain: vec![genesis_block],
            difficulty,
            accounts: vec![],
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
            latest_block.block_hash.clone(),
            txns,
        );

        let mut block_to_mine = new_block;
        block_to_mine.mine_block(self.difficulty);

        // Apply the transactions to the account state before adding the block
        if block_to_mine.apply_transactions(&mut self.accounts) {
            self.chain.push(block_to_mine);
        } else {
            // Panic if the transaction is failed.
            println!("Transaction failed. Block not added.");
        }
    }

    // Check if the blockchain is valid (basic check for now)
    pub fn is_valid(&self) -> bool {
        let length = self.chain.len();
        for i in 1..length {
            let previous_block = &self.chain[i - 1];
            let current_block = &self.chain[i];

            // Check that the previous block's hash matches the current block's "previous_hash"
            if current_block.previous_hash != previous_block.block_hash {
                print!(
                    "previous_hash mismatch: {}, calculated {} ",
                    current_block.previous_hash, previous_block.block_hash
                );
                return false;
            }

            // Check that the current block's hash matches the calculated hash
            let current_recalculated_hash = current_block.calculate_hash();

            if current_block.block_hash != current_recalculated_hash {
                print!(
                    "current_hash mismatch: {}, calculated {} ",
                    current_block.block_hash, current_recalculated_hash
                );
                return false;
            }

            // Check Merkle root consistency
            let transaction_hashes: Vec<String> = current_block
                .transactions
                .clone()
                .into_iter()
                .map(|tx| tx.compute_hash())
                .collect();

            let mut merkle_tree = MerkleTree::new();
            let recalculated_merkle_root = merkle_tree.build_merkle_tree(transaction_hashes);

            // validae if the current block is not already in the tree
            // or if the block is altered o not
            if current_block.merkle_root != recalculated_merkle_root {
                print!(
                    "root mismatch: {}, calculated {} ",
                    current_block.merkle_root, recalculated_merkle_root
                );
                return false;
            }
        }

        // Check if the account state is consistent with the transactions in the block
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

    pub fn genesis_block_details(&mut self) {
        print!("\n genesis_block_details ......:\n");
        let genesis_block = self.chain.first().unwrap();

        println!("- Block Number: {}", genesis_block.block_number);
        println!("- Previous Hash: {}", genesis_block.previous_hash);
        println!("- Merkle Root: {}", genesis_block.merkle_root);
        println!("- Block timestamp: {} \n", genesis_block.timestamp);
        println!("- Block hash: {} \n", genesis_block.block_hash);
        println!("- Transactions:\n");
        for tx in &genesis_block.transactions {
            println!("  - ID: {}", tx.id);
            println!("  - Sender: {}", tx.sender);
            println!("  - Receiver: {}", tx.receiver);
            println!("  - Amount: {}", tx.amount);
            println!("  - Timestamp: {}", tx.timestamp);
        }
        println!("---\n");
    }

    pub fn get_balance(&self, account_address: String) -> Option<f64> {
        self.accounts
            .iter()
            .find(|acc| acc.address == account_address)
            .map(|acc| acc.balance)
    }
}
