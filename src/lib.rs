mod types {
    pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;
}

pub use types::Result;

use std::io::Write;

pub fn get_number(message: &str) -> std::result::Result<i64, std::num::ParseIntError> {
    let mut input = String::new();

    print!("{message}");

    std::io::stdout().flush().expect("Could not flush.");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read line.");

    input.trim().parse::<i64>()
}
