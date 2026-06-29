mod transaction;
mod mempool;
mod block;
mod blockchain;

use transaction::Transaction;
use mempool::Mempool;
use blockchain::Blockchain;

fn main() {

    println!("==============================");
    println!(" Lattice Blockchain v0.6.0");
    println!("==============================");

    let mut chain = Blockchain::new();
    let mut mempool = Mempool::new();

    // 添加交易
    mempool.add_transaction(Transaction::new("Alice", "Bob", 100));
    mempool.add_transaction(Transaction::new("Bob", "Charlie", 25));
    mempool.add_transaction(Transaction::new("Charlie", "David", 10));

    println!("Transactions in mempool: {}", mempool.count());

    // 把 mempool 打包进区块
    chain.add_block(mempool.transactions.clone());

    mempool.transactions.clear();

    chain.print_chain();

    println!();

    if chain.is_valid() {
        println!("✅ Blockchain Valid");
    } else {
        println!("❌ Blockchain Invalid");
    }
}