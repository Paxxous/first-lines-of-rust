use std::io;

const SPECIAL_NUMBER: u32 = 69;

fn main() {
    let result_from_it: bool = loop {
        let mut input = String::new();

        println!("Type in the special number to win.");
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number...");
                continue
            },
        };

        if input == SPECIAL_NUMBER {
            break true;
        }
    };

    if result_from_it == true {
        println!("You did it! You got the correct number, {}", SPECIAL_NUMBER);
    } else {
        println!("Pussy.");
    }
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops
