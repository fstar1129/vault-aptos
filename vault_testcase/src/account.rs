use ed25519_dalek::{ PublicKey, SecretKey};
use rand::{rngs::OsRng, Rng, RngCore, SeedableRng};
use tiny_keccak::{Hasher, Sha3};

pub struct Account {
    pub signing_key: SecretKey,
}


impl Account {
    /// Represents an account as well as the private, public key-pair for the Aptos blockchain.
    pub fn new(priv_key_bytes: Option<Vec<u8>>) -> Self {
        let signing_key = match priv_key_bytes {
            Some(key) => SecretKey::from_bytes(&key).unwrap(),
            None => {
                let mut rng = rand::rngs::StdRng::from_seed(OsRng.gen());
                let mut bytes = [0; 32];
                rng.fill_bytes(&mut bytes);
                SecretKey::from_bytes(&bytes).unwrap()
            }
        };

        Account { signing_key }
    }
    /// Returns the address associated with the given account
    pub fn address(&self) -> String {
        self.auth_key()
    }

    /// Returns the auth_key for the associated account
    pub fn auth_key(&self) -> String {
        let mut sha3 = Sha3::v256();
        sha3.update(PublicKey::from(&self.signing_key).as_bytes());
        sha3.update(&vec![0u8]);

        let mut output = [0u8; 32];
        sha3.finalize(&mut output);
        hex::encode(output)
    }

    /// Returns the public key for the associated account
    pub fn pub_key(&self) -> String {
        hex::encode(PublicKey::from(&self.signing_key).as_bytes())
    }
}