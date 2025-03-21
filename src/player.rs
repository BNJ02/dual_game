/// Structure définissant un joueur.
pub struct Player {
    pub name: String,
    pub vitality: i32,
    pub speed: u64,
    pub strength: i32,
}

impl Default for Player {
    /// Valeurs par défaut d'un joueur.
    fn default() -> Self {
        Self {
            name: "Joueur".into(),
            vitality: 100,
            speed: 50,
            strength: 50,
        }
    }
}
