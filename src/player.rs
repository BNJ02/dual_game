//! Module définissant la structure et les comportements d'un joueur.
//!
//! Ce module fournit la structure [`Player`] ainsi que ses méthodes pour créer un joueur,
//! afficher ses statistiques et appliquer un effet de poison.

use crate::poison::PoisonType;

/// Représente un joueur avec ses caractéristiques.
#[derive(Clone, Debug)]
pub struct Player {
    /// Nom du joueur.
    pub name: String,
    /// Vitalité du joueur.
    pub vitality: u32,
    /// Vitesse du joueur.
    pub speed: u32,
    /// Force du joueur.
    pub strength: u32,
}

impl Player {
    /// Crée un nouveau joueur.
    ///
    /// # Arguments
    ///
    /// * `name` - Le nom du joueur.
    /// * `vitality` - La vitalité initiale du joueur.
    /// * `speed` - La vitesse du joueur.
    /// * `strength` - La force du joueur.
    ///
    /// # Exemples
    ///
    /// ```
    /// use dual_game::player::Player;
    ///
    /// let player = Player::new(String::from("Alice"), 50, 50, 50);
    /// ```
    pub fn new(name: String, vitality: u32, speed: u32, strength: u32) -> Self {
        Player {
            name,
            vitality,
            speed,
            strength,
        }
    }

    /// Affiche les caractéristiques du joueur.
    ///
    /// Cette méthode affiche le nom du joueur et ses statistiques (vitality, speed, strength).
    pub fn display_stats(&self) {
        println!(
            "{} (Vitality={}, Speed={}, Strength={})",
            self.name, self.vitality, self.speed, self.strength
        );
    }

    /// Applique un effet de poison sur le joueur.
    ///
    /// En fonction du type de poison, la vitesse ou la force du joueur est réduite de 5 points,
    /// sans descendre en dessous de zéro.
    ///
    /// # Arguments
    ///
    /// * `poison` - Le type de poison à appliquer.
    pub fn apply_poison(&mut self, poison: PoisonType) {
        match poison {
            PoisonType::Speed => {
                if self.speed >= 5 {
                    self.speed -= 5;
                } else {
                    self.speed = 0;
                }
            }
            PoisonType::Strength => {
                if self.strength >= 5 {
                    self.strength -= 5;
                } else {
                    self.strength = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new(String::from("Test"), 50, 50, 50);
        assert_eq!(player.name, "Test");
        assert_eq!(player.vitality, 50);
    }

    #[test]
    fn test_poison_application_speed() {
        let mut player = Player::new(String::from("Test"), 50, 50, 50);
        player.apply_poison(PoisonType::Speed);
        assert_eq!(player.speed, 45);
    }

    #[test]
    fn test_poison_application_strength() {
        let mut player = Player::new(String::from("Test"), 50, 50, 50);
        player.apply_poison(PoisonType::Strength);
        assert_eq!(player.strength, 45);
    }
}
