use rand::Rng;
use std::cmp::Ordering;
use std::io::{self};

fn main() {
    println!("Guess the number!");
    println!("Please input the upper limit of your desired range:");

    let upper_limit: u32 = loop {
        let mut upper_limit = String::new();

        io::stdin()
            .read_line(&mut upper_limit)
            .expect("Failed to read line!");

        match upper_limit.trim().parse::<u32>() {
            Ok(num) => break num,
            Err(_) => println!("Please enter a valid number!"),
        }
    };

    let secret_number = rand::thread_rng().gen_range(1..=upper_limit);
    let mut number_of_guesses: i32 = 0;

    println!(
        "The secret number has been set between 1 and {}!",
        upper_limit
    );

    loop {
        println!("Please input your guess.");

        let mut user_guess = String::new();
        number_of_guesses += 1;

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", user_guess);

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("It took you {} attempts to guess!", number_of_guesses);
                break;
            }
        }
    }
}
