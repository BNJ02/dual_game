//! Point d'entrée de l'application.
//!
//! Ce module configure et lance la boucle principale du jeu en analysant les arguments en ligne de commande
//! et en créant les joueurs et la partie de jeu correspondante.

use std::error::Error;
use std::io::{stdin, stdout, Write};

use clap::Parser;
use dual_game::game::Game;
use dual_game::player::Player;

/// Structure gérant les arguments en ligne de commande.
///
/// Les paramètres suivants sont disponibles :
/// - `--name1` : Nom du premier joueur.
/// - `--name2` : Nom du deuxième joueur.
/// - `--vitality` : Vitalité initiale des joueurs (défaut: 50).
/// - `--objectifs` : Nombre d’objectifs par tour (défaut: 5).
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Nom du premier joueur
    #[arg(long)]
    name1: String,
    /// Nom du deuxième joueur
    #[arg(long)]
    name2: String,
    /// Vitalité initiale des joueurs (défaut: 50)
    #[arg(long, default_value_t = 50)]
    vitality: u32,
    /// Nombre d’objectifs par tour (défaut: 5)
    #[arg(long, default_value_t = 5)]
    objectifs: usize,
}

/// Fonction principale de l'application.
///
/// Initialise le logger, parse les arguments, crée les joueurs et lance une boucle de parties.
/// L'utilisateur peut choisir de relancer une partie ou de quitter l'application.
fn main() -> Result<(), Box<dyn Error>> {
    // Initialisation du logger (log, env_logger)
    env_logger::init();

    // Parse des arguments en ligne de commande.
    let args: Args = Args::parse();

    // Création des joueurs avec les paramètres fournis.
    let player1 = Player::new(args.name1, args.vitality, 75, 50);
    let player2 = Player::new(args.name2, args.vitality, 75, 50);

    // Boucle principale pour jouer plusieurs parties.
    loop {
        let mut game = Game::new(vec![player1.clone(), player2.clone()], args.objectifs);
        game.run()?;

        println!("\n🔄 Relancer une partie ? [Y/N]");
        loop {
            print!("> ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            match input.trim().to_uppercase().as_str() {
                "Y" => break,
                "N" => return Ok(()),
                _ => println!("Entrée invalide, veuillez entrer Y ou N."),
            }
        }
    }
}
