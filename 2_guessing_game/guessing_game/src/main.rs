use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng - local to the current thread of execution and is seeded by the operating system
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // String::new() - :: an associated function of the String type
    let mut guess = String::new();

    // Could also not import the library and do the function call: std::io::stdin
    // stdin is a function that returns an instance of the library 
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // If the Result is returned as Err - handle with this.
    
    println!("You guessed: {guess}");

    // Compares two numbers 
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

