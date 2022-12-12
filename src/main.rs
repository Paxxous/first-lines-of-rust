use std::io;

fn main() {
    let number = number_input();

    if number == 15 {
        println!("Good choice ;)");
    } else {
        println!("YOUR NUMBER IS NOT FUCKING 15");
    }
}

// I love functions
fn number_input() -> u32 {
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number
        .trim()
        .parse()
        .expect("Please input a number");

    number
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html
