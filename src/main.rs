use std::error::Error;
use std::io::{stdin, stdout, Write};

use clap::Parser;
use dual_game::game::Game;
use dual_game::player::Player;

/// Structure gérant les arguments en ligne de commande.
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

/// Point d’entrée de l’application.
fn main() -> Result<(), Box<dyn Error>> {
    // Initialisation du logger (log, env_logger)
    env_logger::init();

    // Parse des arguments en ligne de commande.
    let args = Args::parse();

    // Création des joueurs. Par défaut, on attribue 50 à la vitalité, à la vitesse et à la force.
    let player1 = Player::new(args.name1, args.vitality, 50, 50);
    let player2 = Player::new(args.name2, args.vitality, 50, 50);

    // Boucle principale pour jouer plusieurs parties.
    loop {
        let mut game = Game::new(vec![player1.clone(), player2.clone()], args.objectifs);
        game.run()?;

        println!("Relancer une partie ? [Y/N]");
        print!("> ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        if !input.trim().eq_ignore_ascii_case("y") {
            break;
        }
    }

    Ok(())
}
