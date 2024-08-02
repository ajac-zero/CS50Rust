use cs50rust::get_string;

fn main() {
    let name = get_string("What's your name? ");

    println!("Hello, {name}");
}
