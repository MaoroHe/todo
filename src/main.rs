use struson::reader::{json_path, simple::*};
use struson::reader::simple::multi_json_path::multi_json_path;
use struson::writer::simple::*;
use std::io::{BufReader, Read};
use std::io;
use std::fs::File;
use std::process::exit;

struct Task {
    id: i32,
    title: String,
    body: String,
    completed: bool
}

fn main() {
    loop {
        let input = command();

        let parsed_input: Result<u32, _> = input.trim().parse();
        match parsed_input {
            Ok(nombre) => return nombre,
            Err(_) => {
                println!("Ce n'est pas un nombre!");
                continue;
            },
        };

        match parsed_input {
            1 => {},
            2 => {},
            3 => {},
            4 => {},
            _ => {},
        }
    }

    let title = "title";

    println!("title: {}", read(0, &title).unwrap_or_else(|e| {
        panic!("Error reading title {}", e);
    }));
}

fn command() {
    std::process::Command::new("clear").status().unwrap();

    println!("Salut! Je suis ton assistant dans la vie de tout les jours, ToDo!");
    println!("Voici une sélection de commande:");
    println!("1. Afficher l'entièreté de nos tâches.");
    println!("2. Crée une nouvelle tâche!");
    println!("3. Supprimer une tâche.");
    println!("4. Quitter le programme.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Il y a une erreur avec la valeur que vous essayer d'input.");

    input
}

fn read(index: i32, precision: &str) -> Result<String, std::io::Error> {
    let mut file = File::open("src/task.json").unwrap();
    let mut result = String::new();

    let mut read = BufReader::new(file);
    let mut item = String::new();
    read.read_to_string(&mut item).unwrap_or_else(|e| {
        panic!("Erreur lors de la lecture du fichier : {:?}", e)
    });

    Ok(item)
}

// std::process::Command::new("clear").status().unwrap();