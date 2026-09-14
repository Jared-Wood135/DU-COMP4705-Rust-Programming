use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Maintain bounds of unguessed values that user potentially will guess from past guesses and
    // process of elimination
    let mut secret_number = rand::thread_rng().gen_range(1..=100);
    let mut lower_bound: u32 = 1;
    let mut upper_bound: u32 = 100;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        // Move the correct number within the bounds of unguessed values (Dynamic correct number)
        // This will maintain the truthfulness of the given hints, but potentially maximize guesses
        if (upper_bound - lower_bound > 3) && (lower_bound <= guess) && (guess <= upper_bound) {
            if guess < secret_number {
                lower_bound = guess;
                secret_number = rand::thread_rng().gen_range(lower_bound+1..upper_bound);
            }
            else {
                upper_bound = guess;
                secret_number = rand::thread_rng().gen_range(lower_bound+1..upper_bound);
            }
        }
    }
}