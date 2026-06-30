use crate::crypto::hash;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub tx_hash: String,
}

impl Transaction {
    pub fn new(from: &str, to: &str, amount: u64) -> Self {
        let mut tx = Self {
            from: from.to_string(),
            to: to.to_string(),
            amount,
            tx_hash: String::new(),
        };

        tx.tx_hash = tx.compute_hash();
        tx
    }

    pub fn compute_hash(&self) -> String {
        hash(&format!("{}{}{}", self.from, self.to, self.amount))
    }
}