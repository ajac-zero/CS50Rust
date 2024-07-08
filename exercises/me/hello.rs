use cs50rust::Result;

fn main() -> Result {
    let mut name = String::new();

    println!("What´s your name? ");

    std::io::stdin().read_line(&mut name)?;

    println!("Hello, {name}");

    Ok(())
}
