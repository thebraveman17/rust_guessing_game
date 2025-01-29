use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut user_guess = String::new();

    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

    let user_guess: u32 = user_guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", user_guess);

    match user_guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
