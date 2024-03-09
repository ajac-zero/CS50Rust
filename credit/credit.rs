use std::io::{self, Write};

fn main() {
    loop {
        let mut input = String::new();

        print!("Number: ");
        io::stdout().flush().expect("Could not flush");

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        match check(&input) {
            true => match luhn(&input) {
                true => {
                    println!("{}", identify(&input));
                    break;
                }
                false => {
                    println!("INVALID");
                    break;
                }
            },
            false => continue,
        }
    }
}

fn check(input: &str) -> bool {
    input.trim().chars().all(char::is_numeric)
}

fn luhn(input: &str) -> bool {
    let (mut even, mut odd) = (0, 0);

    for (idx, char) in input.chars().rev().enumerate() {
        let digit = match char.to_digit(10) {
            Some(d) => d,
            None => continue,
        };
        if idx % 2 == 0 {
            let result = digit * 2;
            for subdigit in result.to_string().chars() {
                match subdigit.to_digit(10) {
                    Some(d) => even += d,
                    None => continue,
                };
            }
        } else {
            odd += digit;
        }
    }

    if (even + odd) % 10 == 0 {
        return true;
    } else {
        return false;
    }
}

fn identify(input: &str) -> &str {
    match &input.trim().len() {
        15 => match &input[0..2] {
            "34" | "37" => return "AMEX",
            _ => return "INVALID",
        },
        16 => match &input[0..2] {
            "51" | "52" | "53" | "54" | "55" => return "MASTERCARD",
            _ => match &input[0..1] {
                "4" => return "VISA",
                _ => return "INVALID",
            },
        },
        13 => match &input[0..1] {
            "4" => return "VISA",
            _ => return "INVALID",
        },
        _ => return "INVALID",
    }
}
