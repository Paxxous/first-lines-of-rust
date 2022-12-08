use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn guessing() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess a number between 1 and 100.");

    // Loop it.
    loop {
        // Take in input via string
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please input a valid string.");

        // Convert and handle errors if anything happens
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            },
        };

        // Compare the input to the secret number
        match input.cmp(&secret_number) {
            Ordering::Less => println!("{} is too small!", input),
            Ordering::Greater => println!("{} is too big!", input),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
    }

}

// The main function 
fn main() {
    guessing();
}

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-invalid-input
