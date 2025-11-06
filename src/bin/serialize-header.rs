use hex;

struct BlockHeader {
    prev_hash: String,
    merkle_root: String,
    timestamp: u32,
    bits: u32,
    nonce: u32,
}

fn main() {

    let block_header = BlockHeader {
        prev_hash: "00000000000000000009b8b8f5c8e2a82b1e3e1f4a2c9d6b8e7f1e0b2c3d4f5a".to_string(),
        merkle_root: "4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e".to_string(),
        timestamp: 1625247600,
        bits: 0x1d00ffff,
        nonce: 0,
    };


    let serialized = _serialize_header(&block_header);
    println!("Serialized Header: {}", hex::encode(&serialized));

}

fn _serialize_header(header: &BlockHeader) -> Vec<u8> {

    let mut bytes = Vec::with_capacity(76);

    let prev_hash_bytes = hex::decode(&header.prev_hash).expect("Decoding prev_hash failed");
    let merkle_root_bytes = hex::decode(&header.merkle_root).expect("Decoding merkle_root failed");

    assert_eq!(prev_hash_bytes.len(), 32);
    assert_eq!(merkle_root_bytes.len(), 32);    


    bytes.extend(prev_hash_bytes);
    bytes.extend(merkle_root_bytes);
    bytes.extend(&header.timestamp.to_le_bytes());
    bytes.extend(&header.bits.to_le_bytes());   
    bytes.extend(&header.nonce.to_le_bytes());

    bytes
}