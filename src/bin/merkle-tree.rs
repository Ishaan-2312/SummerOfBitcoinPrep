use sha2::{Sha256, Digest};
use hex;

fn main() {
    let transactions = vec![
        String::from("tx1"),
        String::from("tx2"),
        String::from("tx3"),
        String::from("tx4"),
    ];

    let merkle_root = create_merkle_tree(transactions);
    println!("Merkle Root: {}", merkle_root);
}


fn sha256_hex(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}


fn create_merkle_tree(transactions: Vec<String>) -> String {

    let mut hashes: Vec<String> = transactions.iter().map(|tx| sha256_hex(tx)).collect();


    if hashes.is_empty() {
        return "".to_string();
    }
    if hashes.len() == 1 {
        return hashes[0].clone();
    }

    if hashes.len() % 2 != 0 {
        hashes.push(hashes.last().unwrap().clone());
    }

    let mut new_hashes = Vec::new();
    for i in (0..hashes.len()).step_by(2) {
        let combined = format!("{}{}", hashes[i], hashes[i + 1]);
        new_hashes.push(sha256_hex(&combined));
    }
    
    create_merkle_tree(new_hashes)


   
}