use std::io;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let answer: u32 = rng.random_range(..100);
    println!("Guess the number between 0 and 100");
    loop {
        println!("Your guess: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let parsed_guess = guess.trim().parse::<u32>().unwrap();
        if parsed_guess < answer {
            println!("Your guess is too low");
        } else if parsed_guess > answer {
            println!("Your guess is too high");
        } else {
            break;
        }
    }
    println!("YOU WON!");
}
