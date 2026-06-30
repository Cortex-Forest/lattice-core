mod blockchain;
mod block;
mod transaction;
mod crypto;
mod mempool;
mod miner;

use std::sync::{Arc, RwLock};

use blockchain::Blockchain;
use mempool::Mempool;
use miner::start_miner;

fn main() {
    println!("🚀 Lattice v5.0 Industrial Consensus Blockchain");

    let chain = Arc::new(RwLock::new(Blockchain::new()));
    let mempool = Arc::new(RwLock::new(Mempool::new()));

    start_miner(chain.clone(), mempool.clone());

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}