//! Module pour le calcul du score.
//!
//! Ce module fournit la structure [`ScoringCalculator`] qui propose des fonctions pour calculer
//! le score en fonction de la différence entre un objectif et une valeur de compteur, du nombre de "miss"
//! et de la force du joueur.

/// Structure pour le calcul du score.
pub struct ScoringCalculator;

impl ScoringCalculator {
    /// Calcule le score pour un objectif donné en fonction de la valeur du compteur,
    /// du nombre de « miss » et de la force du joueur.
    ///
    /// Les règles de calcul sont basées sur la différence absolue entre l’objectif et le compteur :
    /// - Différence == 0         : (100 + force) / (miss+1)
    /// - Différence 1 à 5        : (80 + force) / (miss+1)
    /// - Différence 6 à 10       : (60 + force) / (miss+1)
    /// - Différence 11 à 20      : (40 + force) / (miss+1)
    /// - Différence 21 à 50      : (20 + force) / (miss+1)
    /// - Différence > 50         : (0 + force) / (miss+1)
    ///
    /// # Arguments
    ///
    /// * `objective` - La valeur cible.
    /// * `counter_value` - La valeur atteinte par le compteur.
    /// * `miss` - Le nombre de fois où le compteur s'est réinitialisé (ou "miss").
    /// * `strength` - La force du joueur.
    ///
    /// # Retour
    ///
    /// Retourne le score calculé sous forme de `u32`.
    pub fn calculate_score(objective: u32, counter_value: u32, miss: u32, strength: u32) -> u32 {
        let diff = Self::difference(objective, counter_value);
        let base = if diff == 0 {
            100
        } else if diff <= 5 {
            80
        } else if diff <= 10 {
            60
        } else if diff <= 20 {
            40
        } else if diff <= 50 {
            20
        } else {
            0
        };
        (base + strength) / (miss + 1)
    }

    /// Calcule la moyenne arrondie à l’entier supérieur d'une liste de scores.
    ///
    /// # Arguments
    ///
    /// * `scores` - Un slice de scores (`u32`) à moyenner.
    ///
    /// # Retour
    ///
    /// Retourne la moyenne arrondie à l'entier supérieur.
    pub fn calculate_average(scores: &[u32]) -> u32 {
        let sum: u32 = scores.iter().sum();
        let avg = (sum as f64 / scores.len() as f64).ceil() as u32;
        avg
    }

    /// Calcule la différence entre l'objectif et la valeur du compteur en tenant compte du wrap-around entre 0 et 100.
    ///
    /// Par exemple, pour un objectif de 15 et une valeur de compteur de 95, la différence sera
    /// min(95-15, (100-95)+15) = min(80, 20) = 20.
    ///
    /// # Arguments
    ///
    /// * `objective` - La valeur cible.
    /// * `counter_value` - La valeur atteinte par le compteur.
    ///
    /// # Retour
    ///
    /// Retourne la différence minimale en tenant compte du wrap-around.
    pub fn difference(objective: u32, counter_value: u32) -> u32 {
        let diff = if counter_value >= objective {
            counter_value - objective
        } else {
            objective - counter_value
        };
        let wrap_diff = if counter_value > objective {
            objective + (100 - counter_value)
        } else {
            counter_value + (100 - objective)
        };
        diff.min(wrap_diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference() {
        assert_eq!(ScoringCalculator::difference(15, 95), 20);
        assert_eq!(ScoringCalculator::difference(50, 50), 0);
    }

    #[test]
    fn test_calculate_average() {
        let scores = vec![45, 130, 130, 55, 65];
        // (45+130+130+55+65) = 425, 425/5 = 85
        let avg = ScoringCalculator::calculate_average(&scores);
        assert_eq!(avg, 85);
    }
}
