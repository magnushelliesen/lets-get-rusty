use std::io;
use rand::random_range;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = random_range(1..100);

    println!("Input a number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    println!("Correct number: {secret_number}");
}
