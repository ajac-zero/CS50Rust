use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut input = String::new();

        print!("Choose a number: ");
        io::stdout().flush().expect("Unable to flush");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let num: Result<i8, _> = input.trim().parse();

        match num {
            Ok(n) if n > 0 && n < 9 => {
                build_blocks(n);
                break;
            }
            Ok(_) => println!("Choose only numbers between 1 and 8, please"),
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
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
        println!("");
    }
}
