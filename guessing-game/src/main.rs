use std::io;
use std::cmp::Ordering;

use rand::random_range;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = random_range(1..100);

    println!("Input a number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too low"),
        Ordering::Equal => println!("Correct"),
        Ordering::Greater => println!("Too high"),
    }

    println!("You guessed: {guess}");
    println!("Correct number: {secret_number}");
}
