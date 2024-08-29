extern crate bitcoin;
extern crate hex;

use bitcoin::blockdata::block::Block;
use bitcoin::consensus::encode;
use std::process::Command;
use std::env;
use std::io::Cursor;
use dotenv::dotenv;

fn main() {

    dotenv().ok();
    let rpcuser = env::var("RPC_USER").expect("RPC_USER non défini");
    let rpcpassword = env::var("RPC_PASSWORD").expect("RPC_PASSWORD non défini");
    let blockhash = "000000000000000000000cde9048cd9fb053efee1d31f6636201ac868d2d7cdf";

    // Exécuter la commande bitcoin-cli pour obtenir le bloc en hexadécimal
    let output = Command::new("bitcoin-cli")
        .arg(format!("-rpcuser={}", rpcuser))
        .arg(format!("-rpcpassword={}", rpcpassword))
        .arg("getblock")
        .arg(blockhash)
        .arg("0")
        .output()
        .expect("Erreur lors de l'exécution de bitcoin-cli");

    let block_hex = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let block_bytes = hex::decode(block_hex).expect("Erreur lors de la conversion hexadécimal en octets");
    let cursor = Cursor::new(block_bytes);

    // Utilisation de cursor.get_ref() pour passer &[u8] à la fonction deserialize
    let block: Block = encode::deserialize(cursor.get_ref()).expect("Erreur lors du parsing du bloc");

    // Utilisation de {:?} pour afficher les types qui ne supportent pas Display
    println!("Version: {:?}", block.header.version);
    println!("Previous block hash: {:?}", block.header.prev_blockhash);
    println!("Merkle root: {:?}", block.header.merkle_root);
    println!("Time: {}", block.header.time);
    println!("Bits: {:?}", block.header.bits);
    println!("Nonce: {}", block.header.nonce);
    println!("Nombre de transactions: {}", block.txdata.len());
}
