use std::io::*;

fn main() {
    loop {
        let mut input = String::new();

        print!("Choose a number: ");
        stdout().flush().unwrap();

        stdin().read_line(&mut input).unwrap();

        let num: i8 = input.trim().parse().expect("Invalid input. Integers only please.");

        match num {
            num if num > 0 && num < 9 => {
                build_blocks(num);
                break;
            }
            _ => println!("Choose only numbers between 1 and 8, please"),
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
        blanks = blanks - 1;
        println!("");
    }
}
