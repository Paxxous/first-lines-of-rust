use std::io;

fn main() {
    loop {
        // Grab input from the console
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Translate to int and handle errors
        let input: f64 = match input.trim().parse() {
            Ok(num) => {
                println!("Please input a number thank you!");
                num
            },
            Err(_) => continue,
        };

        println!("You're number plus 69.420 is {}", input + 69.420);

        if input == 0.0 {
            break;
        }
    }

    println!("Thank you for playing");
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
