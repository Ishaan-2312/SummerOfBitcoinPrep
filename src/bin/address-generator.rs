use bitcoin::secp256k1::{Secp256k1, SecretKey};
use bitcoin::{Network, PublicKey, Address};
use rand::rngs::OsRng;

fn main() {

//Generating a random private key first for public key pair
    let secp = Secp256k1::new();
    let mut rng = OsRng;
    let secret_key = SecretKey::new(&mut rng);
    println!("Private Key (hex): {}", secret_key.display_secret());


//Deriving Public key from private key using bitcoin secp256 crate
    let public_key = bitcoin::secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
    println!("Public Key hex : {}", public_key);

//Creating a bitcoin_public viewable key
    let bitcoin_public_key = PublicKey::new(public_key);

//Generating a P2PKH Bitcoin address from the public key
    let address = Address::p2pkh(&bitcoin_public_key, Network::Bitcoin);
    println!("Bitcoin Base58 Address: {}", address);



}