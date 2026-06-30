mod vm;
mod contract;
mod blockchain;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    println!("🚀 Lattice v6.0 Smart Contract VM");

    let mut chain = Blockchain::new();

    // ⭐ 部署合约
    chain.vm.deploy("contract_1".to_string());

    // ⭐ set key-value
    let tx1 = Transaction {
        from: "alice".to_string(),
        to: "contract_1".to_string(),
        method: "set".to_string(),
        args: vec!["name".to_string(), "Alice".to_string()],
    };

    println!("tx1 => {}", chain.process_tx(tx1));

    // ⭐ get value
    let tx2 = Transaction {
        from: "bob".to_string(),
        to: "contract_1".to_string(),
        method: "get".to_string(),
        args: vec!["name".to_string()],
    };

    println!("tx2 => {}", chain.process_tx(tx2));
}