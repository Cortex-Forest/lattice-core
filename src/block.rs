use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub txs: Vec<Transaction>,

    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn genesis() -> Self {
        Self {
            index: 0,
            previous_hash: "0".to_string(),
            txs: vec![],
            nonce: 0,
            hash: "genesis".to_string(),
        }
    }

    pub fn new(index: u64, prev: String, txs: Vec<Transaction>) -> Self {
        let mut block = Self {
            index,
            previous_hash: prev,
            txs,
            nonce: 0,
            hash: String::new(),
        };

        block.mine(4); // difficulty = 4
        block
    }

    pub fn mine(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        loop {
            let hash = self.compute_hash();

            if hash.starts_with(&target) {
                self.hash = hash;
                break;
            }

            self.nonce += 1;
        }
    }

    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();

        hasher.update(self.index.to_string());
        hasher.update(&self.previous_hash);
        hasher.update(self.nonce.to_string());

        for tx in &self.txs {
            hasher.update(&tx.tx_hash);
        }

        hex::encode(hasher.finalize())
    }
}