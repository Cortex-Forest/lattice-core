use crate::transaction::Transaction;

#[derive(Default)]
pub struct Mempool {
    pub txs: Vec<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { txs: vec![] }
    }

    pub fn add(&mut self, tx: Transaction) {
        self.txs.push(tx);
    }

    pub fn drain(&mut self) -> Vec<Transaction> {
        std::mem::take(&mut self.txs)
    }
}