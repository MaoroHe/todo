use struson::reader::{json_path, simple::*};
use struson::reader::simple::multi_json_path::multi_json_path;
use struson::writer::simple::*;
use std::io::{BufReader, Read};
use std::fs::File;

struct Task {
    id: i32,
    title: String,
    body: String,
    completed: bool
}

fn main() {
    loop {
        command();
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
    println!("1. Afficher l'entièreté de nos tâches");
    println!("2. ");
    println!();
    println!();
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