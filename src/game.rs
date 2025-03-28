use std::error::Error;
use std::io::{stdin, stdout, Write};

use crate::objectives::Objectives;
use crate::counter::Counter;
use crate::player::Player;
use crate::scoring::ScoringCalculator;
use crate::poison::{apply_poison, PoisonType};

/// Structure représentant une partie de jeu.
#[derive(Clone, Debug)]
pub struct Game {
    pub players: Vec<Player>,
    pub objectifs_count: usize,
    pub round: u32,
}

impl Game {
    /// Crée une nouvelle partie avec la liste de joueurs et le nombre d’objectifs.
    pub fn new(players: Vec<Player>, objectifs_count: usize) -> Self {
        Game {
            players,
            objectifs_count,
            round: 1,
        }
    }

    /// Exécute la boucle de partie tant que les joueurs ont de la vitalité.
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("##### Démarrage de la partie #####");

        // Boucle tant qu'aucun joueur n'a perdu toute sa vitalité.
        while self.players.iter().all(|p| p.vitality > 0) {
            println!("## Manche {} ##", self.round);

            // Chaque joueur joue son tour.
            let mut scores = Vec::new();
            for i in 0..self.players.len() {
                println!(
                    "Au tour de {} (Vitality={}, Speed={}, Strength={})",
                    self.players[i].name, self.players[i].vitality, self.players[i].speed, self.players[i].strength
                );

                // Génération des objectifs
                let objectives = Objectives::generate(self.objectifs_count);
                println!("→ Objectifs : {:?}", objectives);
                println!("→ Appuyer sur ENTREE pour démarrer le tour..");
                self.wait_enter()?;

                // Exécution du tour et récupération du score moyen.
                let (score, _) = self.play_turn(&objectives, &self.players[i])?;
                println!("→ Score moyen: {}", score);
                scores.push(score);
            }

            // Comparaison des scores entre les deux joueurs.
            if scores.len() < 2 {
                return Err("Nombre de joueurs insuffisant pour déterminer un vainqueur.".into());
            }

            // Traitement en cas d'égalité de scores
            if scores[0] == scores[1] {
                println!("Égalité de scores, aucune pénalité.");
                self.round += 1;
                continue;
            }

            let (winner_index, loser_index) = if scores[0] > scores[1] {
                (0, 1)
            } else {
                (1, 0)
            };

            let diff = scores[winner_index].saturating_sub(scores[loser_index]);
            println!(
                "{} gagne la manche. {} perd {} points de vitalité.",
                self.players[winner_index].name,
                self.players[loser_index].name,
                diff
            );
            self.players[loser_index].vitality = self.players[loser_index].vitality.saturating_sub(diff);

            // Le joueur gagnant choisit quel poison appliquer.
            println!(
                "{} vous devez choisir quel poison appliquer à {} :",
                self.players[winner_index].name, self.players[loser_index].name
            );
            println!("→ 1: -5 speed");
            println!("→ 2: -5 strength");
            let poison_choice = self.get_choice()?;
            let poison_type = match poison_choice {
                1 => PoisonType::Speed,
                2 => PoisonType::Strength,
                _ => {
                    println!("Choix invalide, aucun poison appliqué.");
                    self.round += 1;
                    continue;
                }
            };

            apply_poison(&mut self.players[loser_index], poison_type)?;
            println!("## FIN Manche {} ##", self.round);
            self.round += 1;
        }

        println!("##### Partie terminée #####");
        Ok(())
    }

    /// Lit une ligne depuis l’entrée standard (attend l’appui sur ENTREE).
    fn wait_enter(&self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Ok(())
    }

    /// Lit une option numérique saisie par l’utilisateur.
    fn get_choice(&self) -> Result<u32, Box<dyn Error>> {
        print!("> ");
        stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let choice: u32 = input.trim().parse()?;
        Ok(choice)
    }

    /// Exécute le tour d’un joueur en traitant chacun des objectifs.
    /// Retourne le score moyen obtenu et la liste des scores détaillés.
    pub fn play_turn(&self, objectives: &Vec<u32>, player: &Player) -> Result<(u32, Vec<u32>), Box<dyn Error>> {
        let mut scores = Vec::new();

        // Pour chaque objectif, on simule l'arrêt d'un compteur.
        for (i, obj) in objectives.iter().enumerate() {
            println!("→ Objectif {} : {}", i + 1, obj);
            println!("Appuyez sur ENTREE pour arrêter le compteur");
            self.wait_enter()?;

            // Instanciation d'un compteur utilisant la vitesse du joueur.
            let counter = Counter::new(player.speed);
            // Simulation du comportement du compteur (dans une implémentation réelle, un thread mettrait à jour la valeur)
            let (counter_value, miss) = counter.run(obj.clone());

            let score = ScoringCalculator::calculate_score(*obj, counter_value, miss, player.strength);
            println!("⟹ Counter value = {}, Miss = {} => Score = {}", counter_value, miss, score);
            scores.push(score);
        }
        let average = ScoringCalculator::calculate_average(&scores);
        Ok((average, scores))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn test_game_new() {
        let players = vec![
            Player::new(String::from("Michel"), 50, 50, 50),
            Player::new(String::from("Jacque"), 50, 50, 50),
        ];
        let game = Game::new(players, 5);
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.objectifs_count, 5);
    }
}
