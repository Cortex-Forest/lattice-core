use serde::{Serialize, Deserialize};
use crate::transaction::Transaction;
use crate::block::Block;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Message {
    Transaction(Transaction),
    Block(Block),
}