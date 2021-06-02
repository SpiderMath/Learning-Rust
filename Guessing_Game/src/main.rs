// We are importing the stuff for INPUT & OUTPUT std::io
use std::io;
use rand::Rng;

fn main() {
    let to_be_guessed = rand::thread_rng().gen_range(1..10);
    // The number which is random and we are guessing

    println!("I've guessed a random number from 1 to 10, try and guess it! You got 3 chances");
    // Telling the user to guess the heck up

    // The iterator
    let mut i = 1;

    while i <= 3 {
        // Creates new string
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // I'm SHADOWING the previous value with this!
        let guess: u128 = guess.trim().parse().expect("Please provide a valid number");
        if guess == to_be_guessed {
            println!("Congratulations! You guessed the correct answer!");
            break;
        }
        else {
            if 3 - i != 0 {
                if guess > to_be_guessed {
                    println!("The number is less than that \n Tries Remaining: {}", 3 - i);
                }
                else {
                    println!("The number is more than that \n Tries Remaining: {}", 3 - i);
                }
            }
            else {
                println!("You failed at guessing the number I was thinking of! It was: {}", to_be_guessed);
            }
        }

        i += 1;
    }
}
