use akshually::io::prompt_line;
use rand::{thread_rng, RngCore};
use std::cmp::Ordering;

fn main() {
    println!("Let-s play Guess the Number.");
    let difficulty: usize = prompt_line("Pick a difficulty level (1, 2, or 3): ").unwrap();
    let max = match difficulty {
        1 => 10,
        2 => 100,
        3 => 1000,
        x => panic!("no such difficulty level {x}"),
    };
    let mut rnd = thread_rng();
    loop {
        let number = (rnd.next_u32() % max) as usize;
        play(number);
        let again: Option<String> = prompt_line("Play again? ");
        match again {
            Some(input) => {
                if &input == "y" {
                    continue;
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
}

fn play(number: usize) {
    print!("I have a number. ");
    let mut n_guesses = 0;
    loop {
        let guess: Option<usize> = prompt_line("What's your guess? ");
        n_guesses += 1;
        match guess {
            Some(guess) => match guess.cmp(&number) {
                Ordering::Less => {
                    println!("Too low.");
                    continue;
                }
                Ordering::Greater => {
                    println!("Too high.");
                    continue;
                }
                Ordering::Equal => {
                    println!("You got it in {n_guesses} guesses!");
                    match n_guesses {
                        1 => println!("You're a mind reader!"),
                        2..=4 => println!("Most impressive."),
                        5 | 6 => println!("You can do better than that."),
                        7.. => println!("Better luck next time."),
                        _ => (),
                    }
                    break;
                }
            },
            None => {
                println!("That's not a number!");
                continue;
            }
        }
    }
}
