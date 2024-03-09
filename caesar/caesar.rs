use std::env;
use std::io::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => match &args[1].parse::<u8>() {
            Ok(n) => {
                let output = rotate(&n);
                println!("ciphertext: {}", output);
            }
            Err(_) => println!("Usage: ./caesar key"),
        },
        _ => println!("Usage: ./caesar key"),
    }
}

fn rotate(number: &u8) -> String {
    let mut input = String::new();

    print!("plaintext: ");
    stdout().flush().expect("Could not flush");

    stdin().read_line(&mut input).expect("Could not read line");

    let chars = input.trim().chars();

    let mut output = String::new();

    for c in chars {
        match c as u8 {
            c if c >= 65 && c <= 90 => {
                let value = (c - 65 + number) % 26 + 65;
                output.push(value as char)
            }
            c if c >= 97 && c <= 122 => {
                let value = (c - 97 + number) % 26 + 97;
                output.push(value as char)
            }
            c => output.push(c as char),
        }
    }
    return output;
}
