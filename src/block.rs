use sha2::{Digest, Sha256};
use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub hash: String,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
    ) -> Self {

        let mut hasher = Sha256::new();

        hasher.update(index.to_string());
        hasher.update(&previous_hash);

        for tx in &transactions {
            hasher.update(&tx.from);
            hasher.update(&tx.to);
            hasher.update(tx.amount.to_string());
        }

        let hash = hex::encode(hasher.finalize());

        Self {
            index,
            previous_hash,
            hash,
            transactions,
        }
    }
}