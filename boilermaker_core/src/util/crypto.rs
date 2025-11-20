use hex;
use sha2::{Digest, Sha256};

pub fn sha256_hash_string(s: &str) -> String {
    let result = Sha256::digest(s.as_bytes());
    hex::encode(result)
}
