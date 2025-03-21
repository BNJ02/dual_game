use clap::Parser;
use duel_game::game::play_turn;
use duel_game::player::Player;

/// Arguments pour initialiser les joueurs.
#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "Michel")]
    name1: String,
    #[clap(long, default_value = "Jacque")]
    name2: String,
    #[clap(long, default_value_t = 50)]
    vitality: i32,
    #[clap(long, default_value_t = 50)]
    speed: u64,
    #[clap(long, default_value_t = 50)]
    strength: i32,
    #[clap(long, default_value_t = 5)]
    objectifs: usize,
}


fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut p1 = Player {
        name: args.name1,
        vitality: args.vitality,
        speed: args.speed,
        strength: args.strength,
    };

    let mut p2 = Player {
        name: args.name2,
        vitality: args.vitality,
        speed: args.speed,
        strength: args.strength,
    };

    loop {
        // Tour du joueur 1
        println!("\nAu tour de {} (Vitalité={}, Vitesse={}, Force={})",
            p1.name, p1.vitality, p1.speed, p1.strength);

        let objectifs_p1: Vec<u32> = (0..args.objectifs)
            .map(|_| rand::random::<u32>() % 101)
            .collect();

        let score_p1 = play_turn(&p1, &objectifs_p1);
        println!("→ Score moyen : {}", score_p1);

        p1.vitality -= (score_p1 - p1.strength).abs();

        if p1.vitality <= 0 {
            println!("{} a perdu ! Partie terminée.", p1.name);
            break;
        }

        // Tour du joueur 2
        println!("\nAu tour de {} (Vitalité={}, Vitesse={}, Force={})",
            p2.name, p2.vitality, p2.speed, p2.strength);

        let objectifs_p2: Vec<u32> = (0..args.objectifs)
            .map(|_| rand::random::<u32>() % 101)
            .collect();

        let score_p2 = play_turn(&p2, &objectifs_p2);
        println!("→ Score moyen : {}", score_p2);

        p2.vitality -= (score_p2 - p2.strength).abs();

        if p2.vitality <= 0 {
            println!("{} a perdu ! Partie terminée.", p2.name);
            break;
        }
    }
}
