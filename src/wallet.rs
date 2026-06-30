use ed25519_dalek::{SigningKey, VerifyingKey, Signer};
use rand::rngs::OsRng;

pub struct Wallet {
    pub signing_key: SigningKey,
    pub public_key: VerifyingKey,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let public_key = signing_key.verifying_key();

        Self { signing_key, public_key }
    }

    pub fn address(&self) -> String {
        hex::encode(self.public_key.as_bytes())
    }
}