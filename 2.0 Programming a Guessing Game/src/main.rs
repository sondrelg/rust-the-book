extern crate rand;

use rand::Rng;
use std::io;
use std::process::exit;

fn main() {
    println!("Guess the number!");
    println!("You get five points per correct guess. To exit the application, answer incorrectly or type 'exit'");
    let mut points: u8 = 0;
    let mut done = false;

    while !done {
        let num_one: u8 = rand::thread_rng().gen_range(4, 15); // min 0, max 255 for 8-bit unsigned int
        let num_two: u8 = rand::thread_rng().gen_range(4, 15);

        println!("Please input your guess");
        println!("What is {} x {}?", num_one, num_two);

        // Keyword mut makes variable mutable
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess to int
        let guessed_int: u8 = match guess.trim().parse::<u8>() {
            Ok(n) => n,
            Err(e) => {
                println!("oh no, you typed an illegal value and raised the following error!\nError: {}\nbye bye", e);
                exit(-1)
            }
        };

        // Calculate the correct answer
        let answer: u8 = num_one * num_two;

        if guessed_int == answer {
            if points > 251 {
                println!("You win the game!");
                let done = true;
            }
            points += 5;
            println!("\nCorrect! You now have {} points!", points)
        } else {
            println!("loser");
            done = true;
        }
    }
}