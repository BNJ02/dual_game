use std::io::{self, stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Structure gérant un compteur pour simuler la mécanique d'incrémentation via un thread.
pub struct Counter {
    /// La vitesse détermine la pause (en millisecondes) entre chaque incrémentation.
    pub speed: u32,
}

impl Counter {
    /// Crée un nouveau compteur à partir de la vitesse (qui conditionnera l'incrémentation).
    pub fn new(speed: u32) -> Self {
        Counter { speed }
    }

    /// Exécute le compteur dans un thread pour un objectif donné.
    ///
    /// La logique est la suivante :
    /// - Le compteur s'incrémente toutes les `speed` millisecondes.
    /// - Lorsque le compteur atteint 100, il se réinitialise et le nombre de "miss" est incrémenté.
    /// - En continu, l'état du compteur est affiché, indiquant l'objectif à atteindre, le nombre de "miss" et la valeur actuelle du compteur.
    /// - L'exécution du compteur se termine dès que l'utilisateur appuie sur ENTREE.
    ///
    /// # Paramètres
    /// * `objectif` - La valeur cible qui permet d'établir la différence lors du calcul du score.
    ///
    /// # Retour
    /// * `(counter_value, miss)` - La valeur finale du compteur et le nombre de fois où le compteur a atteint zéro.
    pub fn run(&self, objectif: u32) -> (u32, u32) {
        let (tx, rx) = mpsc::channel();
        let speed = self.speed;

        // Lancement d'un thread pour gérer l'incrémentation du compteur.
        let handle = thread::spawn(move || {
            let mut counter: u32 = 0;
            let mut miss: u32 = 0;
            loop {
                // Terminer la boucle dès que le signal de l'arrêt est reçu.
                if rx.try_recv().is_ok() {
                    return (counter, miss);
                }
                // Affichage de l'état du compteur.
                print!("\r{:<50}\r→ Objectif {} : Miss = {} | Compteur = {}", "", objectif, miss, counter);
                stdout().flush().unwrap();

                // Mise à jour du compteur.
                counter = (counter + 1) % 100;
                if counter == 0 {
                    miss += 1;
                }
                thread::sleep(Duration::from_millis(speed as u64));
            }
        });

        // Attente de l'appui sur ENTREE pour stopper le compteur.
        let mut dummy = String::new();
        let _ = io::stdin().read_line(&mut dummy);

        // Envoi du signal d'arrêt au thread.
        tx.send(()).unwrap();

        let (final_counter, final_miss) = handle.join().unwrap();
        println!(); // Passage à la ligne après la fin du comptage.
        (final_counter, final_miss)
    }

    /// Ancienne implémentation simplifiée qui retournait une valeur calculée à partir de la vitesse.
    /// La méthode `simulate()` est conservée pour compatibilité mais n'est plus utilisée.
    pub fn simulate(&self) -> (u32, u32) {
        let simulated_value = (self.speed * 2) % 101;
        let miss = if simulated_value >= 100 { 1 } else { 0 };
        (simulated_value, miss)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_new() {
        let counter = Counter::new(50);
        assert_eq!(counter.speed, 50);
    }

    // Ce test se contente de vérifier l'initialisation,
    // car tester `run()` nécessiterait de simuler une entrée standard.
    #[test]
    fn test_counter_simulate() {
        let counter = Counter::new(50);
        let (value, miss) = counter.simulate();
        assert!(value <= 100);
        assert!(miss <= 1);
    }
}
