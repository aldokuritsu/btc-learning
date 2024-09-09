use std::process::Command;
use std::env;
use dotenv::dotenv;

pub fn run() {
    dotenv().ok();

    let rpcuser = env::var("RPC_USER").expect("RPC_USER non défini");
    let rpcpassword = env::var("RPC_PASSWORD").expect("RPC_PASSWORD non défini");

    println!("Entrez le hash du bloc que vous voulez récupérer : ");
    let mut blockhash = String::new();
    std::io::stdin().read_line(&mut blockhash).expect("Erreur lors de la lecture de l'entrée");
    let blockhash = blockhash.trim();
    // let blockhash = "000000000000000000000cde9048cd9fb053efee1d31f6636201ac868d2d7cdf";

    let output = Command::new("bitcoin-cli")
        .arg(format!("-rpcuser={}", rpcuser))
        .arg(format!("-rpcpassword={}", rpcpassword))
        .arg("getblockheader")
        .arg(blockhash)
        .output()
        .expect("Erreur lors de l'exécution de bitcoin-cli");

    let block_info = String::from_utf8_lossy(&output.stdout);

    // Afficher directement les informations du bloc récupéré en format JSON
    println!("{}", block_info);
}
