#[derive(Debug)]
struct Block {
    index: u64,
    previous_hash: String,
    hash: String,
    data: String,
}

fn create_genesis_block() -> Block {
    Block {
        index: 0,
        previous_hash: String::from("0"),
        hash: String::from("GENESIS_HASH"),
        data: String::from("Lattice Genesis Block"),
    }
}

fn main() {
    println!("==========================");
    println!("   Lattice Node v0.1");
    println!("==========================");

    let genesis = create_genesis_block();

    println!("Genesis Block:");
    println!("{:#?}", genesis);
}