use sha2::{Sha256, Digest};
use hex;
use num_bigint::BigUint;
use std::str::FromStr;

fn main() {
    let header = "0000000000000000000b4d0b8f3c4e5f6a7b8c9d0e1f2a3b4c5d6e7f8";
    let target_hex = "00000fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

    mine(header, target_hex);
}


fn mine (header : &str , target_hex : &str)  {
    let target = BigUint::parse_bytes(target_hex.as_bytes(), 16).expect("Invalid target hex string");

    let mut nonce = 0u64;

    loop {
        let combined = format!("{}{}",header,nonce);
         
         //New hasher Object
         let mut hasher = Sha256::new();
         //Update for state
         hasher.update(combined.as_bytes());

         //Hash 
        let first_hash = hasher.finalize_reset();

        hasher.update(&first_hash);

        let double_hash = hasher.finalize_reset();

         let hash_hex = hex::encode(&double_hash);

        // Convert Hex to BigUint for Comparison
        let hash_num = BigUint::parse_bytes(hash_hex.as_bytes(), 16).expect("Invalid hash hex string");

        //Compare for Valid Nonce
        if hash_num < target {
            println!("Block mined! Nonce: {}, Hash: {}", nonce, hash_hex);
            break;
        }
        
        //Increase Nonce if not valid
        nonce += 1;
    }



}