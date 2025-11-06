use sha2::{Sha256, Digest};
use hex;

fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

fn merkle_root_in_place(transactions: Vec<String>) -> String {
    // Leaves hashing
    let mut hashes: Vec<String> = transactions.iter().map(|tx| sha256_hex(tx)).collect();
    if hashes.is_empty() {
        return "".to_string();
    }
    // Iteratively process until root
    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            let last = hashes.last().unwrap().clone();
            hashes.push(last);
        }
        let half = hashes.len() / 2;
        for i in 0..half {
            let combined = format!("{}{}", hashes[2 * i], hashes[2 * i + 1]);
            hashes[i] = sha256_hex(&combined);
        }
        hashes.truncate(half); // remove extra elements
    }
    hashes[0].clone()
}

fn main() {
    let transactions = vec![
        "tx1".to_string(),
        "tx2".to_string(),
        "tx3".to_string(),
        "tx4".to_string(),
    ];
    let root = merkle_root_in_place(transactions);
    println!("Merkle Root (in-place): {}", root);
}
