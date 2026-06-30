use crate::{blockchain::Blockchain, mempool::Mempool, transaction::Transaction};
use crate::network::{Message, send, start_server};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Node {
    pub chain: Arc<Mutex<Blockchain>>,
    pub mempool: Arc<Mutex<Mempool>>,
    pub peers: Vec<String>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            chain: Arc::new(Mutex::new(Blockchain::new())),
            mempool: Arc::new(Mutex::new(Mempool::new())),
            peers: vec![],
        }
    }

    pub fn start(self, addr: String) {
        let chain = self.chain.clone();
        let mempool = self.mempool.clone();

        // 🌐 启动 server
        thread::spawn(move || {
            start_server(&addr, move |msg| {
                match msg {
                    Message::Transaction(tx) => {
                        let mut mp = mempool.lock().unwrap();
                        println!("📥 TX received: {}", tx);
                        mp.add(Transaction::new("net", "net", 0));
                    }

                    Message::Block(block) => {
                        let mut ch = chain.lock().unwrap();
                        println!("⛓ Block received: {}", block);
                        // 真实项目这里要验证 + replace chain
                    }

                    Message::RequestChain => {}

                    Message::Chain(c) => {
                        println!("🔄 Chain sync");
                    }
                }
            });
        });
    }

    pub fn broadcast_tx(&self, tx: String) {
        let msg = Message::Transaction(tx);

        for p in &self.peers {
            send(p, &msg);
        }
    }

    pub fn broadcast_block(&self, block: String) {
        let msg = Message::Block(block);

        for p in &self.peers {
            send(p, &msg);
        }
    }
}