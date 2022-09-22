
use std::io;

use rand::Rng;



fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is:{secret_number}");
    println!("Please input your guess.");
    
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("You guessed: {guess}");
}
