use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
}

impl Block {
    pub fn new(
        index: u64,
        previous_hash: String,
        data: String,
    ) -> Self {

        let mut hasher = Sha256::new();

        hasher.update(index.to_string());
        hasher.update(&previous_hash);
        hasher.update(&data);

        let hash = hex::encode(hasher.finalize());

        Self {
            index,
            previous_hash,
            hash,
            data,
        }
    }
}