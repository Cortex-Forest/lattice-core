use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;

pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let keypair = Keypair::generate(&mut csprng);

        Self { keypair }
    }

    pub fn address(&self) -> String {
        hex::encode(self.keypair.public.to_bytes())
    }
}