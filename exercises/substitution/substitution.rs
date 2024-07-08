use std::collections::HashMap;
use std::env;
use std::io::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => cipher(&args[1]),
        _ => println!("Usage: ./substitution key"),
    }
}

fn cipher(key: &String) {
    let uppercase = key.to_uppercase();
    let chars = uppercase.chars();

    let mut map = HashMap::new();
    let mut index: u8 = 65;

    match uppercase.len() {
        26 => {
            for c in chars {
                match c.is_alphabetic() {
                    true => match !map.contains_key(&c) {
                        true => {
                            map.insert(c, index as char);
                            map.insert(c.to_ascii_lowercase(), (index + 32) as char);
                            index += 1;
                        }
                        false => {
                            println!("Repeated key");
                            return;
                        }
                    },
                    false => {
                        println!("Only letters allowed");
                        return;
                    }
                }
            }
        }
        _ => {
            println!("Key must contain 26 characters");
            return;
        }
    }
    let inverted_map: HashMap<char, char> = map.into_iter().map(|(k, v)| (v, k)).collect();
    substitute(inverted_map);
}

fn substitute(map: HashMap<char, char>) {
    let mut input = String::new();
    let mut output = String::new();

    print!("plaintext: ");
    stdout().flush().expect("Could not flush");

    stdin().read_line(&mut input).expect("Could not read lines");

    let chars = input.trim().chars();

    for c in chars {
        match map.get(&c) {
            Some(value) => output.push(*value),
            None => output.push(c),
        }
    }
    println!("ciphertext: {}", output);
}
