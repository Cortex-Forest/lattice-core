mod blockchain;
mod block;
mod transaction;
mod wallet;
mod crypto;
mod mempool;
mod state;
mod network;
mod message;

use state::NodeState;
use network::start_node;

#[tokio::main]
async fn main() {
    println!("🚀 Lattice v4.1 Stable P2P Blockchain");

    let state = NodeState::new();

    start_node("127.0.0.1:9000", state).await;
}