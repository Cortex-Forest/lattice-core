use serde::{Serialize, Deserialize};
use sha2::{Digest, Sha256};

use crate::transaction::Transaction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub txs: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn genesis() -> Self {
        Self {
            index: 0,
            previous_hash: "0".to_string(),
            txs: vec![],
            hash: "genesis".to_string(),
        }
    }

    pub fn new(index: u64, previous_hash: String, txs: Vec<Transaction>) -> Self {
        let mut block = Self {
            index,
            previous_hash,
            txs,
            hash: String::new(),
        };

        block.hash = block.compute_hash();
        block
    }

    pub fn compute_hash(&self) -> String {
        let mut hasher = Sha256::new();

        hasher.update(self.index.to_string());
        hasher.update(&self.previous_hash);

        for tx in &self.txs {
            hasher.update(&tx.tx_hash);
        }

        hex::encode(hasher.finalize())
    }
}