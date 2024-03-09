use std::io;
use std::io::Write;

fn main() {
    let mut name = String::new();

    print!("What´s your name? ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name);

    print!("Hello, {}", name);
}
