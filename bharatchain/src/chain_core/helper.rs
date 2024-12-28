use std::time::{SystemTime, UNIX_EPOCH};

use rand::{rngs::OsRng, RngCore};
use secp256k1::{KeyPair, Secp256k1, SecretKey};

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn generate_key(key: &str) {
    // Initialize the Secp256k1 context
    let secp = Secp256k1::gen_new();

    // Generate 32 random bytes using OsRng
    let mut rng = OsRng;
    let mut secret_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_key_bytes);

    // Create the SecretKey from the random bytes
    let secret_key = SecretKey::from_slice(&secret_key_bytes)
        .expect("Failed to create SecretKey from random bytes");

    // Create a KeyPair from the SecretKey
    let keypair = KeyPair::from_secret_key(&secp, &secret_key)
        .expect("Failed to create KeyPair from SecretKey");

    // Print the secret key
    println!("Secret key: {:?}", secret_key);

    // Print the public key
    let public_key = keypair.public_key();
    println!("Public key: {:?}", public_key);
}
