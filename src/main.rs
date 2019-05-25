use std::io;

fn main() {
    println!("Number Guesser");
    
    println!("Please type in a guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read Line");

    println!("Your guess: {}", guess);
}