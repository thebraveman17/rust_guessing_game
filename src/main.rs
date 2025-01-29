use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut user_guess = String::new();

    io::stdin()
        .read_line(&mut user_guess)
        .expect("Failed to read line");

    println!("You guessed: {}", user_guess);
}
