use cs50rust::get_number;

fn main() {
    loop {
        let num = get_number("Choose a number between 1 and 8: ").unwrap_or(0);

        if 0 < num && num < 9 {
            build_blocks(num);
            break;
        } else {
            println!("Choose only numbers between 1 and 8, please");
        }
    }
}

fn build_blocks(blocks: i64) {
    for n in 1..=blocks {
        for _ in 1..=(blocks - n) {
            print!(" ");
        }
        for _ in 1..=n {
            print!("#");
        }
        println!("");
    }
}
