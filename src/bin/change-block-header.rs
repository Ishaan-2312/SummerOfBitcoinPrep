use sha2::{Sha256, Digest};
use hex;


struct BlockHeader {
    prev_hash: String,
    merkle_root: String,
    timestamp: u32,
    bits: u32,
    nonce: u32,
}

fn main() {

    let blockHeader : BlockHeader = BlockHeader {
        prev_hash: "0000000000000000000b4d0b8f3c4e5f6a7b8c9d0e1f2a3b4c5d6e7f8".to_string(),
        merkle_root: "4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e".to_string(),
        timestamp: 1625247600,
        bits: 0x1d00ffff,
        nonce: 0,
    };

    let blockHeader2 : BlockHeader = BlockHeader {
        prev_hash: "0000000000000000000b4d0b8f3c4e5f6a7b8c9d0e1f2a3b4c5d6e7f8".to_string(),
        merkle_root: "4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5f".to_string(),
        timestamp: 1625247600,
        bits: 0x1d00ffff,
        nonce: 0,
        
    };

    let hash = block_hash(&blockHeader);
    let hash2 = block_hash(&blockHeader2);
    println!("Block Hash: {}", hash);
    println!("Block Hash 2: {}", hash2);

}

fn block_hash(header : &BlockHeader) -> String {

    let  mut hasher = Sha256::new();
    let header_data = format!("{}{}{}{}{}", header.prev_hash, header.merkle_root, header.timestamp, header.bits, header.nonce);
    hasher.update(header_data.as_bytes());
    let first_hash = hasher.finalize_reset();
    hasher.update(&first_hash);
    let double_hash = hasher.finalize_reset();

     
    let hash_hex = hex::encode(&double_hash);
    hash_hex
}