use crate::block::Block;
use crate::transaction::Transaction;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {

    pub fn new() -> Self {
        let genesis = Block::new(
            0,
            "0".to_string(),
            vec![],
        );

        Self {
            blocks: vec![genesis],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous = self.blocks.last().unwrap();

        let block = Block::new(
            previous.index + 1,
            previous.hash.clone(),
            transactions,
        );

        self.blocks.push(block);
    }

    pub fn print_chain(&self) {
        println!("\n========== Lattice Blockchain ==========");

        for block in &self.blocks {
            println!("{:#?}", block);
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            let recalculated = Block::new(
                current.index,
                current.previous_hash.clone(),
                current.transactions.clone(),
            );

            if current.hash != recalculated.hash {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }

        true
    }
}