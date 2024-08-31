use std::io::{self, Write}; // Pour gérer les entrées/sorties

mod parser1;
mod parser2;


fn main() {
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
    
}