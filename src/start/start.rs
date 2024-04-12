pub mod letStart {
    pub fn start() {
        std::process::Command::new("clear").status().unwrap();
    
        println!("Salut! Je suis ton assistant dans la vie de tout les jours, ToDo!");
        println!("Voici une sélection de commande:");
        println!("1. Afficher l'entièreté de nos tâches.");
        println!("2. Crée une nouvelle tâche!");
        println!("3. Supprimer une tâche.");
        println!("4. Quitter le programme.");
    }
}