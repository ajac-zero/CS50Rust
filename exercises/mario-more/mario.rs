use cs50rust::{parse_number, Result};

fn main() -> Result {
    loop {
        let mut input = String::new();

        println!("Choose a number: ");
        std::io::stdin().read_line(&mut input)?;

        let result = parse_number(&input);

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

fn build_blocks(blocks: i64) {
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
