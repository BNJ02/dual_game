/// Calcul le score selon la différence absolue, les miss, et la force.
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
