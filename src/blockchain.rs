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
}