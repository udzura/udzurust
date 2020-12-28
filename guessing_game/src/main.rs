extern crate rand;

use std::io::stdin;
use std::cmp::Ord;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let nr = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", nr);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(nr) => nr,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&nr) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
