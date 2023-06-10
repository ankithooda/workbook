use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("##### Guessing Game #####");

    loop {
        println!("Enter your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline from stdin.");

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("You guessed low.") }
            Ordering::Greater => { println!("You guessed high.") }
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        }
    }
}
