use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering::{Less, Greater, Equal};

fn main() {
    let answer: u32 = rand::rng().random_range(0..100);
    println!("Guess the number between 0 and 100");
    loop {
        println!("Your guess: ");
        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");
        let parsed_guess = guess.trim().parse::<u32>().expect("Please type a number!");

        match parsed_guess.cmp(&answer) {
            Less => println!("Too low"),
            Greater => println!("Too high"),
            Equal => {
                println!("You win!!!");
                break;
            }
        };
    }
}
