use sha2::{Digest, Sha256};
use crate::transaction::Transaction;
use crate::merkle::merkle_root;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,

    pub merkle_root: String,
    pub nonce: u64,
    pub difficulty: usize,

    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        transactions: Vec<Transaction>,
        difficulty: usize,
    ) -> Self {

        let merkle_root = merkle_root(
            transactions.iter()
                .map(|t| t.tx_hash.clone())
                .collect()
        );

        let mut block = Self {
            index,
            previous_hash,
            timestamp: now(),
            transactions,
            merkle_root,
            nonce: 0,
            difficulty,
            hash: String::new(),
        };

        block.mine();
        block
    }

    fn mine(&mut self) {
        let target = "0".repeat(self.difficulty);

        loop {
            let hash = self.calculate_hash();

            if hash.starts_with(&target) {
                self.hash = hash;
                break;
            }

            self.nonce += 1;
        }
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}{}",
            self.index,
            self.previous_hash,
            self.timestamp,
            self.merkle_root,
            self.nonce,
            self.difficulty
        );

        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }
}

fn now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}