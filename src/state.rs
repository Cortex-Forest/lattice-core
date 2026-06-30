use std::sync::{Arc, RwLock};

use crate::blockchain::Blockchain;
use crate::mempool::Mempool;

#[derive(Clone)]
pub struct NodeState {
    pub chain: Arc<RwLock<Blockchain>>,
    pub mempool: Arc<RwLock<Mempool>>,
}

impl NodeState {
    pub fn new() -> Self {
        Self {
            chain: Arc::new(RwLock::new(Blockchain::new())),
            mempool: Arc::new(RwLock::new(Mempool::new())),
        }
    }
}