use cs50rust::get_number;

fn main() {
    loop {
        let change = get_number("Changed owed: ");

        match change {
            Ok(n) if n >= 0 => {
                let coins = get_coin(n, 0);
                println!("{coins}");
                break;
            }
            Ok(_) => println!("Only positive numbers allowed"),
            Err(_) => println!("Only numbers allowed"),
        }
    }
}

fn get_coin(n: i64, coins: i64) -> i64 {
    let (quarter, dime, nickel, penny) = (25, 10, 5, 1);

    let remainder = if n >= quarter {
        Some(n - quarter)
    } else if n >= dime {
        Some(n - dime)
    } else if n >= nickel {
        Some(n - nickel)
    } else if n >= penny {
        Some(n - penny)
    } else {
        None
    };

    match remainder {
        Some(r) => get_coin(r, coins + 1),
        None => coins,
    }
}
