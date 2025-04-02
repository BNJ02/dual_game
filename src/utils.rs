//! Module utilitaire pour le calcul du score.
//!
//! Ce module fournit une fonction de calcul du score basée sur la différence absolue,
//! le nombre de "miss" et la force.

/// Calcule le score selon la différence absolue, le nombre de miss et la force.
///
/// Les règles de calcul sont les suivantes :
/// - Différence == 0         : (100 + strength) / (miss+1)
/// - Différence de 1 à 5      : (80 + strength) / (miss+1)
/// - Différence de 6 à 10     : (60 + strength) / (miss+1)
/// - Différence de 11 à 20    : (40 + strength) / (miss+1)
/// - Différence de 21 à 50    : (20 + strength) / (miss+1)
/// - Différence > 50          : strength / (miss+1)
///
/// # Arguments
///
/// * `diff` - La différence absolue entre l'objectif et la valeur du compteur.
/// * `miss` - Le nombre de "miss".
/// * `strength` - La force du joueur, représentée sous forme d'un entier.
///
/// # Retour
///
/// Retourne le score calculé sous forme d'un `i32`.
pub fn calculate_score(diff: u32, miss: u32, strength: i32) -> i32 {
    match diff {
        0 => (100 + strength) / (miss as i32 + 1),
        1..=5 => (80 + strength) / (miss as i32 + 1),
        6..=10 => (60 + strength) / (miss as i32 + 1),
        11..=20 => (40 + strength) / (miss as i32 + 1),
        21..=50 => (20 + strength) / (miss as i32 + 1),
        _ => strength / (miss as i32 + 1),
    }
}
