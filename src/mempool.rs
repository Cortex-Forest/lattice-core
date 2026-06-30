use crate::transaction::Transaction;

pub struct Mempool {
    pub pending: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            pending: vec![],
        }
    }

    pub fn add(&mut self, tx: Transaction) {
        self.pending.push(tx);
    }

    pub fn drain(&mut self) -> Vec<Transaction> {
        let txs = self.pending.clone();
        self.pending.clear();
        txs
    }

    pub fn size(&self) -> usize {
        self.pending.len()
    }
}