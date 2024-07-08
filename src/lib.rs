mod types {
    pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;
}

pub use types::Result;

pub fn parse_number(input: &str) -> std::result::Result<i64, std::num::ParseIntError> {
    input.trim().parse::<i64>()
}
