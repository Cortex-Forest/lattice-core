use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::sync::{Arc, RwLock};

use crate::state::NodeState;
use crate::message::Message;

pub async fn start_node(addr: &str, state: NodeState) {
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🌐 Node running at {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let state = state.clone();

        tokio::spawn(async move {
            let mut buf = vec![0; 4096];

            let n = socket.read(&mut buf).await.unwrap();
            let data = String::from_utf8_lossy(&buf[..n]);

            if let Ok(msg) = serde_json::from_str::<Message>(&data) {
                match msg {
                    Message::Transaction(tx) => {
                        let mut mp = state.mempool.write().unwrap();
                        mp.add(tx);
                    }

                    Message::Block(block) => {
                        let mut chain = state.chain.write().unwrap();

                        let last = chain.blocks.last().unwrap();

                        if block.previous_hash == last.hash {
                            chain.blocks.push(block);
                            println!("📦 new block accepted");
                        }
                    }
                }
            }
        });
    }
}