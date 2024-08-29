use bitcoin::blockdata::block::Block;
use bitcoin::consensus::encode;
use std::process::Command;
use std::env;
use std::io::Cursor;
use dotenv::dotenv;

fn main() {
    // Charger les variables d'environnement depuis un fichier `.env` (s'il existe)
    dotenv().ok();

    // Récupérer les informations de connexion RPC (nom d'utilisateur et mot de passe) depuis les variables d'environnement
    let rpcuser = env::var("RPC_USER").expect("RPC_USER non défini");
    let rpcpassword = env::var("RPC_PASSWORD").expect("RPC_PASSWORD non défini");

    // Définir le hash du bloc que nous voulons récupérer
    let blockhash = "000000000000000000000cde9048cd9fb053efee1d31f6636201ac868d2d7cdf";

    // Exécuter la commande `bitcoin-cli` pour obtenir le bloc correspondant au hash
    // Le résultat est renvoyé en hexadécimal
    let output = Command::new("bitcoin-cli")
        .arg(format!("-rpcuser={}", rpcuser))       // Spécifie l'utilisateur RPC
        .arg(format!("-rpcpassword={}", rpcpassword)) // Spécifie le mot de passe RPC
        .arg("getblock")                             // Commande pour obtenir un bloc
        .arg(blockhash)                              // Le hash du bloc à récupérer
        .arg("0")                                    // Niveau de détail du bloc : 0 signifie retour en hexadécimal
        .output()                                    // Exécuter la commande
        .expect("Erreur lors de l'exécution de bitcoin-cli"); // Gérer l'erreur si la commande échoue

    // Convertir la sortie de la commande (qui est en bytes) en une chaîne de caractères
    let block_hex = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Décoder la chaîne hexadécimale en bytes pour pouvoir la manipuler
    let block_bytes = hex::decode(block_hex).expect("Erreur lors de la conversion hexadécimal en octets");

    // Créer un curseur pour lire les bytes, utile pour la désérialisation
    let cursor = Cursor::new(block_bytes);

    // Désérialiser les bytes en une structure de bloc Bitcoin (le type `Block` de la crate `bitcoin`)
    // `cursor.get_ref()` retourne une référence au vecteur de bytes, nécessaire pour la désérialisation
    let block: Block = encode::deserialize(cursor.get_ref()).expect("Erreur lors du parsing du bloc");

    // Afficher les informations du bloc récupéré

    // La version du bloc est un nombre qui indique les règles de validation spécifiques à ce bloc
    println!("Version: {:?}", block.header.version);

    // L'identifiant du bloc précédent dans la chaîne (utilisé pour valider la continuité de la blockchain)
    println!("Previous block hash: {:?}", block.header.prev_blockhash);

    // Le Merkle root est un résumé cryptographique de toutes les transactions contenues dans le bloc
    println!("Merkle root: {:?}", block.header.merkle_root);

    // L'horodatage du bloc (quand le bloc a été miné)
    println!("Time: {}", block.header.time);

    // Les bits représentent la cible actuelle pour la difficulté de minage
    println!("Bits: {:?}", block.header.bits);

    // Le nonce est un nombre que les mineurs modifient pour essayer de trouver un bloc valide
    println!("Nonce: {}", block.header.nonce);

    // Le nombre de transactions dans le bloc
    println!("Nombre de transactions: {}", block.txdata.len());
}
