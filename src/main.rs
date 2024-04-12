use std::io::{BufReader, Read};
use std::io;
use std::fs::File;
use struson::reader::simple::*;
use struson::reader::simple::multi_json_path::multi_json_path;

struct Task {
    id: i32,
    title: String,
    body: String,
    completed: bool
}

fn main() {
    start();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Il y a une erreur avec la valeur que vous essayer d'input.");

        let input: u32 = match input.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => {
                println!("Veuillez entrer un nombre entre 1 et 4.");
                continue
            },
        };

        match input {
            1 => {
                // fonction read
            },
            2 => {
                // fonction create
            },
            3 => {
                // fonction delete
            },
            4 => {
                println!("Programme en cours d'arrêt...");
                std::process::exit(0)
            },
            _ => {
                println!("Cette commande n'existe pas!");
                continue
            }
        }
    }
}

fn read() {
    let mut file = File::open("src/task.json").unwrap();
    let mut result = String::new();

    let mut read = BufReader::new(file);
    let mut item = String::new();
    read.read_to_string(&mut item).unwrap_or_else(|e| {
        panic!("Erreur lors de la lecture du fichier : {:?}", e)
    });

    let json_reader = SimpleJsonReader::new(item.as_bytes());
}

// std::process::Command::new("clear").status().unwrap();