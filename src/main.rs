mod blockchain;
mod block;
mod transaction;
mod merkle;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {

    println!("⛏️ Lattice v1.0 Mining Chain");

    let mut chain = Blockchain::new();

    let tx1 = Transaction::new("Alice", "Bob", 100);
    let tx2 = Transaction::new("Bob", "Charlie", 50);
    let tx3 = Transaction::new("Charlie", "Alice", 20);

    println!("Mining block 1...");
    chain.add_block(vec![tx1, tx2]);

    println!("Mining block 2...");
    chain.add_block(vec![tx3]);

    chain.print_chain();

    println!();

    if chain.is_valid() {
        println!("✅ Chain Valid");
    } else {
        println!("❌ Chain Invalid");
    }
}