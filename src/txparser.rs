use std::process::Command;
use std::env;
use std::io;
use dotenv::dotenv;
use serde_json::Value;

pub fn run() {
    // Charger les variables d'environnement depuis un fichier `.env` (s'il existe)
    dotenv().ok();

    // Récupérer les informations de connexion RPC (nom d'utilisateur et mot de passe) depuis les variables d'environnement
    let rpcuser = env::var("RPC_USER").expect("RPC_USER non défini");
    let rpcpassword = env::var("RPC_PASSWORD").expect("RPC_PASSWORD non défini");

    // Demander à l'utilisateur d'entrer l'ID de la transaction qu'il souhaite examiner
    println!("Entrez l'ID de la transaction que vous voulez récupérer : ");
    let mut txid = String::new();
    io::stdin().read_line(&mut txid).expect("Erreur lors de la lecture de l'entrée");
    let txid = txid.trim();  // Supprimer les espaces blancs et les nouvelles lignes

// Demander à l'utilisateur d'entrer le hash du bloc contenant la transaction
    println!("Entrez le hash du bloc contenant la transaction : ");
    let mut blockhash = String::new();
    io::stdin().read_line(&mut blockhash).expect("Erreur lors de la lecture de l'entrée");
    let blockhash = blockhash.trim();  // Supprimer les espaces blancs et les nouvelles lignes


    // Exécuter la commande `bitcoin-cli` pour obtenir la transaction en hexadécimal
    let output = Command::new("bitcoin-cli")
        .arg(format!("-rpcuser={}", rpcuser))       // Spécifie l'utilisateur RPC
        .arg(format!("-rpcpassword={}", rpcpassword)) // Spécifie le mot de passe RPC
        .arg("getrawtransaction")                   // Commande pour obtenir une transaction brute
        .arg(txid)                                  // L'ID de la transaction à récupérer
        .arg("1")                                   // Niveau de détail : 0 signifie retour en hexadécimal
        .arg(blockhash)                                    
        .output()                                   // Exécuter la commande
        .expect("Erreur lors de l'exécution de bitcoin-cli");

    // Convertir la sortie de la commande (qui est en bytes) en une chaîne de caractères JSON
    let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parser la chaîne JSON pour extraire les informations
    let tx_data: Value = serde_json::from_str(&output_str).expect("Erreur lors du parsing de la transaction JSON");

    // Afficher les informations de la transaction récupérée
    let version = tx_data["version"].as_i64().unwrap();
    let num_inputs = tx_data["vin"].as_array().unwrap().len();
    let num_outputs = tx_data["vout"].as_array().unwrap().len();
    let lock_time = tx_data["locktime"].as_i64().unwrap();

    println!("Version de la transaction : {}", version);
    println!("Nombre d'inputs : {}", num_inputs);
    println!("Nombre d'outputs : {}", num_outputs);
    println!("Lock time : {}", lock_time);
}

