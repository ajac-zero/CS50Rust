use cs50rust::Result;
use std::io::Write;

fn main() -> Result {
    loop {
        let mut input = String::new();

        print!("Choose a number: ");

        std::io::stdout().flush()?;

        std::io::stdin().read_line(&mut input)?;

        let result = input.trim().parse::<i8>();

        match result {
            Ok(num) if 0 < num && num < 9 => {
                build_blocks(num);
                break;
            }
            _ => println!("Choose only numbers between 1 and 8, please"),
        }
    }

    Ok(())
}

fn build_blocks(blocks: i8) {
    let mut blanks = blocks - 1;
    for n in 1..=blocks {
        for _ in 1..=blanks {
            print!(" ");
        }
        for _ in 1..=n {
            print!("#");
        }
        print!("  ");
        for _ in 1..=n {
            print!("#");
        }
        blanks = blanks - 1;
        print!("\n");
    }
}
