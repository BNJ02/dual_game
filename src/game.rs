use crate::{player::Player, utils::calculate_score};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Gère un tour de jeu pour un joueur.
pub fn play_turn(player: &Player, objectifs: &[u32]) -> i32 {
    let mut scores = vec![];

    println!("→ Objectifs : {:?}", objectifs);
    println!("→ Appuyer sur ENTREE pour démarrer le tour...");

    for &objectif in objectifs { // Correction ici (&objectif au lieu de objectif)
        let (tx, rx) = mpsc::channel();
        let speed = player.speed;

        let handle = thread::spawn(move || {
            let mut counter = 0;
            let mut miss = 0;
            loop {
                if rx.try_recv().is_ok() {
                    return (counter, miss);
                }
                print!("\r{:width$}\r→ Objectif {} : Miss = {} | Compteur = {}", "", objectif, miss, counter, width = 50);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();

                counter = (counter + 1) % 100;
                if counter == 0 {
                    miss += 1;
                }
                thread::sleep(Duration::from_millis(speed));
            }
        });

        let _ = std::io::stdin().read_line(&mut String::new());
        tx.send(()).unwrap();

        let (final_counter, miss): (u32, u32) = handle.join().unwrap();
        let diff = final_counter.abs_diff(objectif);
        let score = calculate_score(diff, miss, player.strength);
        println!(" | Score obtenu : {}", score);
        scores.push(score);
    }

    let moyenne: f64 = scores.iter().sum::<i32>() as f64 / scores.len() as f64;
    moyenne.ceil() as i32
}
