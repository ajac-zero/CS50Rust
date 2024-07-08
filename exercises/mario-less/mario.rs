use cs50rust::{get_number, Result};

fn main() -> Result {
    loop {
        let num = get_number("Choose a number betweeb 1 and 8: ")?;

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

fn build_blocks(blocks: i64) {
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
