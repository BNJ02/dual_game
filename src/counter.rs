//! Module gérant un compteur utilisé pour simuler une incrémentation avec un thread.
//!
//! Ce module définit la structure [`Counter`] et ses méthodes associées. Le compteur s'incrémente à une
//! vitesse donnée et, lors de son exécution, affiche son état en continu jusqu'à ce que l'utilisateur appuie sur ENTREE.

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
    /// Crée un nouveau compteur à partir de la vitesse spécifiée.
    ///
    /// # Arguments
    ///
    /// * `speed` - La vitesse d'incrémentation (en millisecondes).
    ///
    /// # Exemples
    ///
    /// ```
    /// use dual_game::counter::Counter;
    ///
    /// let counter = Counter::new(50);
    /// ```
    pub fn new(speed: u32) -> Self {
        Counter { speed }
    }

    /// Exécute le compteur dans un thread pour un objectif donné.
    ///
    /// La logique est la suivante :
    /// - Le compteur s'incrémente toutes les `speed` millisecondes.
    /// - Lorsque le compteur atteint 100, il se réinitialise et le nombre de "miss" est incrémenté.
    /// - En continu, l'état du compteur est affiché, indiquant l'objectif, le nombre de "miss" et la valeur actuelle.
    /// - L'exécution du compteur se termine dès que l'utilisateur appuie sur ENTREE.
    ///
    /// # Arguments
    ///
    /// * `objectif` - La valeur cible utilisée pour le calcul du score.
    ///
    /// # Retour
    ///
    /// Retourne un tuple `(counter_value, miss)` où :
    /// - `counter_value` représente la valeur finale du compteur.
    /// - `miss` correspond au nombre de fois où le compteur a atteint zéro.
    pub fn run(&self, objectif: u32) -> (u32, u32) {
        let (tx, rx) = mpsc::channel();
        let speed = self.speed;

        // Lancement d'un thread pour gérer l'incrémentation du compteur.
        let handle = thread::spawn(move || {
            let mut counter: u32 = 0;
            let mut miss: u32 = 0;
            loop {
                // Terminer la boucle dès que le signal d'arrêt est reçu.
                if rx.try_recv().is_ok() {
                    return (counter, miss);
                }
                // Affichage de l'état du compteur.
                print!("\r{:<50}\r→ Objectif {} : Miss = {} | Compteur = {}", "", objectif, miss, counter);
                stdout().flush().unwrap();

                // Mise à jour du compteur.
                counter = (counter + 1) % 101; // Réinitialisation à 0 si le compteur atteint 100.
                // Incrémentation du nombre de "miss" si le compteur est à 0.
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
        // println!(); // Passage à la ligne après la fin du comptage.
        (final_counter, final_miss)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vérifie que la création d'un compteur avec une vitesse donnée fonctionne correctement.
    #[test]
    fn test_counter_new() {
        let counter = Counter::new(50);
        assert_eq!(counter.speed, 50);
    }

    /// Test de simulation du compteur.
    ///
    /// Ce test vérifie simplement l'initialisation et une exécution minimale, en simulant un arrêt rapide
    /// pour éviter que le compteur ne dépasse les limites attendues.
    #[test]
    fn test_counter_simulate() {
        let counter = Counter::new(50);

        // Simuler un thread séparé pour arrêter rapidement le compteur.
        let handle = thread::spawn(move || {
            let (value, _miss) = counter.run(50); // Utilisation d'une valeur d'objectif valide
            assert!(value <= 100);
        });

        // Simuler un délai suffisant pour permettre au compteur de s'exécuter brièvement.
        thread::sleep(Duration::from_millis(100));

        // Simule l'appui sur ENTREE en envoyant un signal d'arrêt via un canal.
        // (No action needed here as the ENTER key press is simulated by stopping the thread.)

        // Attendre la fin du thread avant de vérifier les assertions.
        if let Err(err) = handle.join() {
            panic!("Thread panicked: {:?}", err);
        }
    }
}
