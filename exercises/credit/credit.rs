use cs50rust::{get_string, Result};

fn main() -> Result {
    loop {
        let input = get_string("Number: ");

        if check(&input) {
            if luhn(&input) {
                println!("{}", identify(&input));
                break;
            } else {
                println!("INVALID");
                break;
            }
        }
    }

    Ok(())
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
        true
    } else {
        false
    }
}

fn identify(input: &str) -> &str {
    match &input.trim().len() {
        15 => match &input[0..2] {
            "34" | "37" => "AMEX",
            _ => "INVALID",
        },
        16 => match &input[0..2] {
            "51" | "52" | "53" | "54" | "55" => "MASTERCARD",
            _ => match &input[0..1] {
                "4" => "VISA",
                _ => "INVALID",
            },
        },
        13 => match &input[0..1] {
            "4" => "VISA",
            _ => "INVALID",
        },
        _ => "INVALID",
    }
}
