use cs50rust::Result;

fn main() -> Result {
    loop {
        let mut input = String::new();

        println!("Changed owed: ");

        std::io::stdin().read_line(&mut input)?;

        let result = input.trim().parse::<u32>()?;

        match result {
            n if n >= 0 => {
                let coins = change(n, 0);
                println!("{}", coins);
                break;
            }
            n => println!("Only positive numbers allowed"),
            _ => println!("Only numbers allowed"),
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
