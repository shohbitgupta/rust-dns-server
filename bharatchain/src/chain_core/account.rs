use hex::decode;
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug)]
pub struct Account {
    pub address: String,
    pub balance: f64,
}

impl Account {
    // Constructor to create a new account with a given address and initial balance.
    pub fn from_secret_key(secret_key: &str, balance: f64) -> Result<Self, String> {
        // Create a new Secp256k1 context
        let secret_key_bytes = decode(secret_key).map_err(|e| format!("Invalid hex: {}", e))?;

        // Ensure the secret key is exactly 32 bytes (standard for secp256k1)
        if secret_key_bytes.len() != 32 {
            return Err("Secret key must be 32 bytes long...".to_string());
        }

        // Convert the bytes into a SecretKey
        let secret_key = SecretKey::from_slice(&secret_key_bytes)
            .map_err(|e| format!("Invalid secret key: {}", e))?;

        // Generate the corresponding public key
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);

        // Derive the address from the public key (hash of public key)
        let pub_key_bytes = public_key.serialize_uncompressed();
        let address = format!("{:x}", Sha256::digest(pub_key_bytes.as_ref()));

        print!("Account address: {} \n *********** ==== *********", address); // Use the public key hash as address

        Ok(Account { address, balance })
    }

    // Method to debit the account (subtract balance).
    pub fn debit(&mut self, amount: f64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    // Method to credit the account (add balance).
    pub fn credit(&mut self, amount: f64) {
        self.balance += amount;
    }
}
