use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::blockchain::Blockchain;
use crate::mempool::Mempool;

pub fn start_miner(chain: Arc<RwLock<Blockchain>>, mempool: Arc<RwLock<Mempool>>) {
    thread::spawn(move || loop {

        let txs = {
            let mut mp = mempool.write().unwrap();
            mp.drain()
        };

        if txs.is_empty() {
            thread::sleep(Duration::from_millis(500));
            continue;
        }

        {
            let mut chain = chain.write().unwrap();
            chain.add_block(txs);

            println!("⛏ New block mined!");
        }
    });
}