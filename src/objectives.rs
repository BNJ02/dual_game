use rand::Rng;
use std::collections::HashMap;

/// Module pour la génération d’objectifs.
pub struct Objectives;

impl Objectives {
    /// Génère un vecteur d'objectifs aléatoires entre 0 et 100.
    pub fn generate(n: usize) -> Vec<u32> {
        let mut rng = rand::rng();
        (0..n).map(|_| rng.random_range(0..=100)).collect()
    }
    
    /// Variante bonus : génère une HashMap associant une touche à un objectif.
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

    #[test]
    fn test_generate_objectives() {
        let objs = Objectives::generate(5);
        assert_eq!(objs.len(), 5);
        for obj in objs {
            assert!(obj <= 100);
        }
    }
    
    #[test]
    fn test_generate_map() {
        let map = Objectives::generate_map(5);
        // Le nombre d'éléments peut être inférieur à 5 si des clés se chevauchent.
        assert!(map.len() <= 5);
    }
}
