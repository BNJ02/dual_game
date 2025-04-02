//! Module définissant la logique de la partie de jeu.
//!
//! Ce module contient la structure [`Game`] qui gère les tours de jeu, le calcul des scores et l'application
//! des effets de poison entre les joueurs.

use std::error::Error;
use std::io::{Write, stdin, stdout};

use crate::counter::Counter;
use crate::objectives::Objectives;
use crate::player::Player;
use crate::poison::{PoisonType, apply_poison};
use crate::scoring::ScoringCalculator;

/// Structure représentant une partie de jeu.
#[derive(Clone, Debug)]
pub struct Game {
    /// Liste des joueurs participant à la partie.
    pub players: Vec<Player>,
    /// Nombre d’objectifs par tour.
    pub objectifs_count: usize,
    /// Numéro du tour courant.
    pub round: u32,
}

impl Game {
    /// Crée une nouvelle partie avec la liste de joueurs et le nombre d’objectifs par tour.
    ///
    /// # Arguments
    ///
    /// * `players` - Un vecteur contenant les joueurs.
    /// * `objectifs_count` - Le nombre d’objectifs à générer pour chaque tour.
    ///
    /// # Exemples
    ///
    /// ```
    /// use dual_game::game::Game;
    /// use dual_game::player::Player;
    ///
    /// let players = vec![
    ///     Player::new(String::from("Alice"), 50, 50, 50),
    ///     Player::new(String::from("Bob"), 50, 50, 50),
    /// ];
    /// let game = Game::new(players, 5);
    /// ```
    pub fn new(players: Vec<Player>, objectifs_count: usize) -> Self {
        Game {
            players,
            objectifs_count,
            round: 1,
        }
    }

    /// Exécute la boucle de la partie tant que tous les joueurs ont encore de la vitalité.
    ///
    /// Chaque tour se compose des actions suivantes :
    /// - Affichage du numéro de tour.
    /// - Chaque joueur joue son tour, ce qui inclut la génération d'objectifs et l'exécution d'un tour de jeu.
    /// - Les scores sont comparés pour déterminer le gagnant du tour.
    /// - Le joueur perdant subit une pénalité de vitalité.
    /// - Le gagnant choisit un effet de poison à appliquer au perdant.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si la partie s'est terminée normalement ou une erreur dans le cas contraire.
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        println!("##### Démarrage de la partie #####");

        // Boucle tant qu'aucun joueur n'a perdu toute sa vitalité.
        while self.players.iter().all(|p| p.vitality > 0) {
            println!("\n## Manche {} ##", self.round);

            // Chaque joueur joue son tour.
            let mut scores = Vec::new();
            for i in 0..self.players.len() {
                if i > 0 {
                    println!();
                }
                println!(
                    "Au tour de {} (Vitality={}, Speed={}, Strength={})",
                    self.players[i].name,
                    self.players[i].vitality,
                    self.players[i].speed,
                    self.players[i].strength
                );

                // Génération des objectifs.
                let objectives = Objectives::generate(self.objectifs_count);
                println!("→ Objectifs : {:?}", objectives);
                println!("→ Appuyer sur ENTREE pour démarrer le tour..");
                self.wait_enter()?;

                // Exécution du tour et récupération du score moyen.
                let (score, _) = self.play_turn(&objectives, &self.players[i])?;
                println!("\n# Fin du tour #");
                println!("→ Score moyen: {}", score);
                scores.push(score);
            }

            // Comparaison des scores entre les joueurs.
            if scores.len() < 2 {
                return Err("Nombre de joueurs insuffisant pour déterminer un vainqueur.".into());
            }

            // Traitement en cas d'égalité de scores.
            if scores[0] == scores[1] {
                println!("\nÉgalité de scores, aucune pénalité.");
                self.round += 1;
                continue;
            }

            // Détermination du gagnant et du perdant.
            let (winner_index, loser_index) = if scores[0] > scores[1] {
                (0, 1)
            } else {
                (1, 0)
            };

            let diff = scores[winner_index].saturating_sub(scores[loser_index]);
            println!(
                "\n{} gagne la manche. {} perd {} points de vitalité.",
                self.players[winner_index].name, self.players[loser_index].name, diff
            );
            self.players[loser_index].vitality =
                self.players[loser_index].vitality.saturating_sub(diff);

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

        println!("\n##### Partie terminée #####");
        Ok(())
    }

    /// Attend que l'utilisateur appuie sur ENTREE.
    ///
    /// Cette méthode lit une ligne depuis l'entrée standard et permet de faire une pause dans le déroulement du tour.
    ///
    /// # Retour
    ///
    /// Retourne `Ok(())` si la lecture s'est déroulée sans problème.
    fn wait_enter(&self) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        Ok(())
    }

    /// Lit et valide le choix numérique de l'utilisateur.
    ///
    /// Cette méthode demande à l'utilisateur de saisir 1 ou 2 et continue de redemander en cas d'entrée invalide.
    ///
    /// # Retour
    ///
    /// Retourne le choix de l'utilisateur sous forme de `u32`.
    fn get_choice(&self) -> Result<u32, Box<dyn Error>> {
        loop {
            print!("> ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let trimmed = input.trim();
            if let Ok(choice) = trimmed.parse::<u32>() {
                if choice == 1 || choice == 2 {
                    return Ok(choice);
                }
            }
            println!("Entrée invalide, veuillez entrer 1 ou 2.");
        }
    }

    /// Exécute le tour d’un joueur en traitant chacun des objectifs.
    ///
    /// Pour chaque objectif, un compteur est lancé et la méthode [`Counter::run`] est appelée pour simuler
    /// le comportement du compteur. Le score est ensuite calculé en fonction de la valeur du compteur,
    /// du nombre de "miss" et de la force du joueur.
    ///
    /// # Arguments
    ///
    /// * `objectives` - Une référence vers un vecteur d'objectifs numériques.
    /// * `player` - Le joueur dont le tour est en cours.
    ///
    /// # Retour
    ///
    /// Retourne un tuple `(score_moyen, scores_détaillés)` :
    /// - `score_moyen` est le score moyen obtenu lors du tour.
    /// - `scores_détaillés` est un vecteur contenant les scores de chaque objectif.
    pub fn play_turn(
        &self,
        objectives: &[u32],
        player: &Player,
    ) -> Result<(u32, Vec<u32>), Box<dyn Error>> {
        let mut scores = Vec::new();

        // Pour chaque objectif, on simule l'arrêt d'un compteur.
        for obj in objectives.iter() {
            // Instanciation d'un compteur utilisant la vitesse du joueur.
            let counter = Counter::new(player.speed);
            // Simulation du comportement du compteur.
            let (counter_value, miss) = counter.run(*obj);

            let score =
                ScoringCalculator::calculate_score(*obj, counter_value, miss, player.strength);
            // println!("⟹ Counter value = {}, Miss = {} => Score = {}", counter_value, miss, score);
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

    /// Vérifie que la création d'une nouvelle partie avec deux joueurs et un nombre d'objectifs donné fonctionne.
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
