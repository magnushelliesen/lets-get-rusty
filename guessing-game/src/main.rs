use std::cmp::Ordering;
use std::io;

use rand::random_range;

fn main() {
    println!("Welcome to the guessing game!");

    let secret_number = random_range(1..100);

    println!("Input a number: ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low."),
            Ordering::Greater => println!("Too high."),
            Ordering::Equal => {
                println!("Correct, the secret number is {secret_number}.");
                break;
            }
        }
    }

    println!("Congratulations! The game is now over.")
}
