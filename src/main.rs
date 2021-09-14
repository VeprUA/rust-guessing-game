extern crate rand;

use std::io;

fn get_guess() -> u8 {
    loop {
        println!("Input guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read from stdin");

        // Use trim to get rid of any new lines or spaces that would cause
        // the parsing to fail.
        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(err) => {
                println!("Couldn't understand input: {}", err);
            }
        }
    }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess < correct {
        println!("Too Low!");
        false
    } else if guess > correct {
        println!("Too High!");
        false
    } else {
        println!("You got it!");
        true
    }
}

fn main() {
    let actual_value = rand::random::<u8>();

    println!("Welcome to the guessing game!");
    loop {
        let guess = get_guess();
        if handle_guess(guess, actual_value) {
            break;
        }
    }
}
