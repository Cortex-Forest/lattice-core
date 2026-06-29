use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    previous_hash: String,
    hash: String,
    data: String,
}

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let genesis = Block::new(0, "0".to_string(), "Lattice Genesis Block".to_string());

        Blockchain {
            blocks: vec![genesis],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous = self.blocks.last().unwrap();

        let block = Block::new(
            previous.index + 1,
            previous.hash.clone(),
            data,
        );

        self.blocks.push(block);
    }

    fn print_chain(&self) {
        println!("\n========== Lattice Blockchain ==========");

        for block in &self.blocks {
            println!("{:#?}", block);
        }
    }
}

impl Block {
    fn new(index: u64, previous_hash: String, data: String) -> Self {

        let mut hasher = Sha256::new();

        hasher.update(index.to_string());
        hasher.update(&previous_hash);
        hasher.update(&data);

        let hash = hex::encode(hasher.finalize());

        Block {
            index,
            previous_hash,
            hash,
            data,
        }
    }
}

fn main() {

    println!("==============================");
    println!(" Lattice Blockchain v0.2");
    println!("==============================");

    let mut chain = Blockchain::new();

    chain.add_block("Alice -> Bob : 100 LAT".to_string());

    chain.add_block("Bob -> Charlie : 25 LAT".to_string());

    chain.add_block("Charlie -> David : 10 LAT".to_string());

    chain.print_chain();

}