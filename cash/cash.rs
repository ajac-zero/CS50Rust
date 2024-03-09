use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut input = String::new();

        print!("Changed owed: ");
        io::stdout().flush().expect("Could not flush");

        io::stdin()
            .read_line(&mut input)
            .expect("Could not read line");

        let result: Result<i8, _> = input.trim().parse();

        match result {
            Ok(n) if n >= 0 => {
                let coins = change(n, 0);
                println!("{}", coins);
                break;
            }
            Ok(_) => println!("Only positive numbers allowed"),
            Err(_) => println!("Only numbers allowed"),
        }
    }
}

fn change(mut n: i8, mut coins: i8) -> i8 {
    let (quarter, dime, nickel, penny) = (25, 10, 5, 1);

    if n >= quarter {
        n = n - quarter;
        coins = coins + 1;
        return change(n, coins);
    } else if n >= dime {
        n = n - dime;
        coins = coins + 1;
        return change(n, coins);
    } else if n >= nickel {
        n = n - nickel;
        coins = coins + 1;
        return change(n, coins);
    } else if n >= penny {
        n = n - penny;
        coins = coins + 1;
        return change(n, coins);
    } else {
        return coins;
    }
}
