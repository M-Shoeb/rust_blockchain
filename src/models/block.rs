use super::blockchain::Blockchain;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    // The index in which the current block is stored.
    pub index: u64,
    // The time the current block is created.
    pub timestamp: u64,
    // The block's proof of work.
    pub proof_of_work: u64,
    // The previous block hash.
    pub previous_hash: String,
    // The current block hash.
    pub hash: String,
}

impl Block {
    // Create a new block. The hash will be calculated and set automatically.
    pub fn new(index: u64, previous_hash: String) -> Self {
        // Current block to be created.
        let block = Block {
            index,
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(),
            previous_hash,
            hash: String::default(),
        };

        block
    }

    // Mine block hash.
    pub fn mine(&mut self, blockchain: Blockchain) {
        loop {
            // The following line checks if the hash field of the current block does not start with a string of "0" characters repeated blockchain.difficulty times. This condition is used to determine if the block's hash satisfies the desired mining difficulty level specified by the Blockchain object.
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)) {
                self.proof_of_work += 1; // This line increments the proof_of_work field of the current block by 1. The proof_of_work value is typically used in the mining process to find a valid block hash.
                self.hash = self.generate_block_hash(); //This line updates the hash field of the current block by generating a new block hash using the generate_block_hash method. It is assumed that this method exists in the Block struct to calculate the hash based on the block's data.
            } else {
                break;
            }
        }
    }

    // Calculate block hash.
    pub fn generate_block_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        // Convert block to JSON format.
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();

        // Calculate and return SHA-256 hash value.
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);

        let result = hasher.finalize();

        format!("{:x}", result)
    }
}
