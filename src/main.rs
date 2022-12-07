use std::io;
use std::cmp::Ordering;
use rand::Rng;

// The main function 
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("{secret_number}");

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Please input a valid string.");

        let converted_input: u32 = input
            .trim()
            .parse()
            .expect("Please input a number.");

        match converted_input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
    }

    // https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
}
