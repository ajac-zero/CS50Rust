use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");

    let words: Vec<_> = input.split(' ').collect();
    let sentences: Vec<_> = input.split(|c| c == '!' || c == '?' || c == '.').collect();
    let letters: Vec<_> = input.chars().filter(|&c| c.is_alphabetic()).collect();

    let n_letters = letters.len() as f64;
    let n_words = words.len() as f64;
    let n_sentences = (sentences.len() - 1) as f64;

    let L = (n_letters / n_words) * 100.0;
    let S = (n_sentences / n_words) * 100.0;

    let index = 0.0588 * L - 0.296 * S - 15.8;

    if index < 1.0 {
        println!("Before Grade 1");
    } else if index > 15.0 {
        println!("Grade 16+");
    } else {
        println!("Grade {}", index.round());
    }
}
