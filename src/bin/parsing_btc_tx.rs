use bitcoin::consensus::encode::deserialize;
use bitcoin::Transaction;
use hex;

fn main(){
    let raw_tx = "010000000001014ac4f30fe22e983b4985b01552fe35098bb7fb32193c24c673814bfe11f9e1da0000000000ffffffff02d0f90300000000001600141912d8f302c32ef75899698854929f6430b7ec6090ec00000000000016001445997b06d4ffdca6acab6f63f0dc3ee8ab14e037040047304402204f98033069fe15456c8cf6199fd63e2509b9bfbaa8a540fea58153698978345a02205584147aef1f41517225b8bc1c77fa688e425fa5b431b00ed3e6b99f112ee7930147304402205129171a4e0260edfc541a7a0ccaf47bb5a2758d78145711486fc829250a7c8a0220466925d4f10ab19546155b83bc16392a1817dd71ab9068d8080d93e940330ae5014752210248cc91ea5287a0cccdad7ab09f192228f0e7c35282560995b79f8bfa1a61fa4d21024aff956beecc7e5ca479080b20335a4559e47a0cfd29734fc0cd221f8a42ca4752ae00000000";
    let raw_bytes = hex::decode(raw_tx).unwrap();

    let tx: Transaction = deserialize(&raw_bytes).unwrap();

    println!("TX version: {:?}", tx.version);
    println!("TX lock_time: {}", tx.lock_time);

    for (i, input) in tx.input.iter().enumerate() {
        println!("\nInput {}", i);
        println!("  Prev txid: {}", input.previous_output.txid);
        println!("  Prev vout: {}", input.previous_output.vout);
        println!("  ScriptSig: {}", input.script_sig);
        println!("  Sequence: {}", input.sequence);
        println!("{:?}",input.witness);
        
    }

    for (i, output) in tx.output.iter().enumerate() {
        println!("\nOutput {}", i);
        println!("  Value: {} sat", output.value);
        println!("  ScriptPubKey: {}", output.script_pubkey);
    }

}