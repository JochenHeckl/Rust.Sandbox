use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Make your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Please input numbers only.");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Try again, guess higher!"),
            std::cmp::Ordering::Equal => {
                println!("You guessed correct!");
                break;
            }
            std::cmp::Ordering::Greater => println!("Try again, guess lower!"),
        }
    }
}
