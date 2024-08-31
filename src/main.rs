use std::io::{self, Write}; // Pour gérer les entrées/sorties
use std::env; // Pour gérer les arguments

mod parser1;
mod parser2;


fn main() {

// Utilisation des arguments pour déterminer le script à exécuter
let args: Vec<String> = env::args().collect();


if args.len() < 2 {
    println!("\\nQuel script voulez-vous exécuter ?\\n");
    println!("1. Parser1");
    println!("2. Parser2");
    println!("3. Quitter");

    print!("Votre choix : ");
        io::stdout().flush().unwrap(); // S'assurer que l'invite s'affiche
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        // Voici un hash valide : 000000000000000000000cde9048cd9fb053efee1d31f6636201ac868d2d7cdf
        match choice.trim() {
            "1" => {
                // Appel du parser1 
                parser1::run();
            }
            "2" => {
                // Appel du parser2 
                parser2::run();
            }
            "3" => {
                // Appel du parser1 
                println!("Ok, bye !");
            }
            _ => println!("Choix invalide !"),
        }
        } else {

        match args[1].as_str() {
            "parser1" => parser1::run(),
            "parser2" => parser2::run(),
            _ => println!("Commande non reconnue : {}", args[1]),
        }
    }
}