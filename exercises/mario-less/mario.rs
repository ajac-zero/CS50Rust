use cs50rust::Result;

fn main() -> Result {
    loop {
        let mut input = String::new();

        println!("Choose a number: ");
        std::io::stdin().read_line(&mut input)?;

        let num = input.trim().parse::<i8>()?;

        match num {
            num if 0 < num && num < 9 => {
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
        blanks = blanks - 1;
        print!("\n");
    }
}
