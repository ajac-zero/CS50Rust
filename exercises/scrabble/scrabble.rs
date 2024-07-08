use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let points = HashMap::from([
        ('A', 1),
        ('B', 3),
        ('C', 3),
        ('D', 2),
        ('E', 1),
        ('F', 4),
        ('G', 2),
        ('H', 4),
        ('I', 1),
        ('J', 8),
        ('K', 5),
        ('L', 1),
        ('M', 3),
        ('N', 1),
        ('O', 1),
        ('P', 3),
        ('Q', 10),
        ('R', 1),
        ('S', 1),
        ('T', 1),
        ('U', 1),
        ('V', 4),
        ('W', 4),
        ('X', 8),
        ('Y', 4),
        ('Z', 10),
    ]);

    let player1 = get_input(1);
    let player2 = get_input(2);

    let score1 = get_points(player1, &points);
    let score2 = get_points(player2, &points);

    if score1 > score2 {
        println!("Player 1 wins!");
    } else if score2 > score1 {
        println!("Player 2 wins!")
    } else {
        println!("Tie!")
    }
}

fn get_input(player: i8) -> String {
    let mut input = String::new();

    print!("Player {}: ", player);
    io::stdout().flush().expect("Could not flush");

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    return input;
}

fn get_points(mut input: String, points: &HashMap<char, i8>) -> i8 {
    let mut total = 0;
    input = input.to_uppercase();
    for character in input.chars() {
        match points.get(&character) {
            Some(value) => {
                total = total + value;
            }
            None => continue,
        }
    }
    return total;
}
