mod block;
mod blockchain;

mod transaction;
mod wallet;
mod crypto;
mod network;
mod consensus;

use blockchain::Blockchain;

fn main() {

    println!("==============================");
    println!(" Lattice Blockchain v0.4");
    println!("==============================");

    let mut chain = Blockchain::new();

    chain.add_block("Alice -> Bob : 100 LAT".to_string());

    chain.add_block("Bob -> Charlie : 25 LAT".to_string());

    chain.add_block("Charlie -> David : 10 LAT".to_string());

    chain.print_chain();

    println!();

    if chain.is_valid() {

        println!("✅ Blockchain Valid");

    } else {

        println!("❌ Blockchain Invalid");

    }

}