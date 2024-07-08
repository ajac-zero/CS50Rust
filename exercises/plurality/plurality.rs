use std::env;
use std::io;

struct Candidate {
    name: String,
    votes: u8,
}

fn main() {
    let MAX = 9;
    let candidates: [Candidate; 9];

    let args: Vec<String> = env::args().collect();
    if 2 > args.len() {
        println!("Usage: plurality [candidate ...]")
    }

    let candidate_count = args.len() - 1;
    if candidate_count > MAX {
        println!("Maximum number of candidates is {}", MAX)
    }

    for i in 0..candidate_count {
        candidates[i].name = args[i + 1];
        candidates[i].votes = 0
    }

    let mut input = String::new();
    io::stdin.read_line(&mut input).expect("Could not read line.")

    let voter_count = input.parse().expect("Could not parse")
    for i  in 0..voter_count {
        input = ''
        io::stdin.read_line(&mut input).expect("Could not read line.")
        if !vote(&input, candidate_count, &candidates) {
            println!("Invalid vote.")
        }
    }
}

fn vote(input: &String, candidate_count: u8, candidates: &[Candidate; 9]) {
    for i in 0..candidate_count {
        if candidates[i].name == input {
            candidates[i].votes += 1
        }
    }
}
