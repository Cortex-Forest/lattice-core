use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = Self {
            blocks: vec![],
            difficulty: 4, // 👈 挖矿难度（可调）
        };

        let genesis = Block::new(
            0,
            "0".to_string(),
            vec![],
            chain.difficulty,
        );

        chain.blocks.push(genesis);
        chain
    }

    pub fn add_block(&mut self, txs: Vec<Transaction>) {
        let prev = self.blocks.last().unwrap();

        let block = Block::new(
            prev.index + 1,
            prev.hash.clone(),
            txs,
            self.difficulty,
        );

        self.blocks.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let cur = &self.blocks[i];
            let prev = &self.blocks[i - 1];

            if cur.hash != cur.calculate_hash() {
                return false;
            }

            if cur.previous_hash != prev.hash {
                return false;
            }
        }

        true
    }

    pub fn print_chain(&self) {
        for b in &self.blocks {
            println!("{:#?}", b);
        }
    }
}