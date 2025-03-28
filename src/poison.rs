use crate::player::Player;

/// Énumération des types de poison pouvant être appliqués.
#[derive(Clone, Debug)]
pub enum PoisonType {
    Speed,
    Strength,
}

/// Applique l’effet de poison sur le joueur cible.
/// Le poison modifie soit la vitesse, soit la force du joueur.
///
/// # Arguments
/// * `target` - Le joueur cible sur lequel appliquer le poison.
/// * `poison_type` - Le type de poison à appliquer.
///
/// # Retour
/// * `Ok(())` si l’opération s’est déroulée correctement.
/// * `Err(String)` dans le cas d’une erreur (rare dans cette implémentation simple).
pub fn apply_poison(target: &mut Player, poison_type: PoisonType) -> Result<(), String> {
    target.apply_poison(poison_type);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;

    #[test]
    fn test_apply_poison_speed() {
        let mut player = Player::new(String::from("Test"), 50, 50, 50);
        // Appliquer -5 speed
        assert_eq!(player.speed, 50);
        apply_poison(&mut player, PoisonType::Speed).unwrap();
        assert_eq!(player.speed, 45);
    }

    #[test]
    fn test_apply_poison_strength() {
        let mut player = Player::new(String::from("Test"), 50, 50, 50);
        // Appliquer -5 strength
        assert_eq!(player.strength, 50);
        apply_poison(&mut player, PoisonType::Strength).unwrap();
        assert_eq!(player.strength, 45);
    }
}
