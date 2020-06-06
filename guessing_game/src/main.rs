use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);

    println!("The guessing game has begun! (1-100)");

    loop {
        println!("Please enter a number to guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Guess is {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("The guessed number is too low"),
            Ordering::Greater => println!("The guessed number is too high"),
            Ordering::Equal => {
                println!("The guessed number is correct!");
                break;
            }
        };
    }
}
