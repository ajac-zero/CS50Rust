use std::io::*;

fn main() {
    let mut name = String::new();

    print!("What´s your name? ");
    stdout().flush().unwrap();

    stdin().read_line(&mut name).unwrap();

    print!("Hello, {name}");
}
