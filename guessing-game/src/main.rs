use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");
    println!("Input a number:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {guess}");
}
