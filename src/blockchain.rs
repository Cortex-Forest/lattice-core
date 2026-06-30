use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, txs: Vec<Transaction>) {
        let prev = self.blocks.last().unwrap();

        let block = Block::new(
            prev.index + 1,
            prev.hash.clone(),
            txs,
        );

        self.blocks.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let cur = &self.blocks[i];
            let prev = &self.blocks[i - 1];

            if cur.previous_hash != prev.hash {
                return false;
            }

            if cur.hash != cur.compute_hash() {
                return false;
            }
        }
        true
    }

    // ⭐ v5.0 核心：链比较（最长链）
    pub fn replace_chain(&mut self, new_chain: Vec<Block>) {
        if new_chain.len() > self.blocks.len() {
            self.blocks = new_chain;
            println!("🔄 Chain replaced (longest chain rule)");
        }
    }
}