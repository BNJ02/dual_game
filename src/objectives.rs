//! Module pour la génération d’objectifs.
//!
//! Ce module fournit des fonctions permettant de générer des objectifs sous forme de vecteurs ou de maps.
//! Ces objectifs sont utilisés pour définir des cibles aléatoires dans le jeu.

use rand::Rng;
use std::collections::HashMap;

/// Structure regroupant les fonctions de génération d’objectifs.
pub struct Objectives;

impl Objectives {
    /// Génère un vecteur d'objectifs aléatoires.
    ///
    /// Chaque objectif est une valeur numérique comprise entre 0 et 100.
    ///
    /// # Arguments
    ///
    /// * `n` - Le nombre d'objectifs à générer.
    ///
    /// # Retour
    ///
    /// Retourne un vecteur de `u32` contenant les objectifs générés.
    ///
    /// # Exemples
    ///
    /// ```
    /// use dual_game::objectives::Objectives;
    ///
    /// let objs = Objectives::generate(5);
    /// assert_eq!(objs.len(), 5);
    /// ```
    pub fn generate(n: usize) -> Vec<u32> {
        let mut rng = rand::rng();
        (0..n).map(|_| rng.random_range(0..=100)).collect()
    }
    
    /// Génère une `HashMap` associant une touche à un objectif.
    ///
    /// Chaque clé est une lettre aléatoire et la valeur correspondante est un objectif aléatoire entre 0 et 100.
    ///
    /// # Arguments
    ///
    /// * `n` - Le nombre d'associations à générer.
    ///
    /// # Retour
    ///
    /// Retourne une `HashMap<char, u32>` contenant les associations générées.
    ///
    /// # Remarque
    ///
    /// Le nombre d'éléments dans la map peut être inférieur à `n` si des clés se chevauchent.
    pub fn generate_map(n: usize) -> HashMap<char, u32> {
        let mut rng = rand::rng();
        // Liste de lettres pouvant être utilisées comme clés.
        let keys = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
        let mut map = HashMap::new();
        for _ in 0..n {
            let key = keys[rng.random_range(0..keys.len())];
            let value = rng.random_range(0..=100);
            map.insert(key, value);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vérifie que la génération d'un vecteur d'objectifs fonctionne correctement.
    #[test]
    fn test_generate_objectives() {
        let objs = Objectives::generate(5);
        assert_eq!(objs.len(), 5);
        for obj in objs {
            assert!(obj <= 100);
        }
    }
    
    /// Vérifie que la génération d'une map d'objectifs fonctionne correctement.
    #[test]
    fn test_generate_map() {
        let map = Objectives::generate_map(5);
        // Le nombre d'éléments peut être inférieur à 5 si des clés se chevauchent.
        assert!(map.len() <= 5);
    }
}
