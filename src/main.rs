mod blockchain;
mod block;
mod transaction;
mod merkle;
mod mempool;

use blockchain::Blockchain;
use transaction::Transaction;
use mempool::Mempool;

fn main() {

    println!("🚀 Lattice v1.1 Mempool Chain");

    let mut chain = Blockchain::new();
    let mut mempool = Mempool::new();

    // 👇 模拟用户不断发交易（进入 mempool）
    let tx1 = Transaction::new("Alice", "Bob", 100);
    let tx2 = Transaction::new("Bob", "Charlie", 50);
    let tx3 = Transaction::new("Charlie", "Alice", 20);
    let tx4 = Transaction::new("David", "Alice", 5);

    mempool.add(tx1);
    mempool.add(tx2);
    mempool.add(tx3);
    mempool.add(tx4);

    println!("📦 Mempool size: {}", mempool.size());

    // ⛏️ 矿工开始打包
    chain.mine_pending(&mut mempool);

    // 再来一轮
    let tx5 = Transaction::new("Alice", "Eve", 30);
    mempool.add(tx5);

    chain.mine_pending(&mut mempool);

    println!("\n===== CHAIN =====");
    chain.print_chain();

    println!();

    if chain.is_valid() {
        println!("✅ Chain Valid");
    } else {
        println!("❌ Chain Invalid");
    }
}