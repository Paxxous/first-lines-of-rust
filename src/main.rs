use std::io;

fn main() {
    loop {
        let mut str = String::new();
        io::stdin()
            .read_line(&mut str)
            .expect("Failed to read line");

        // Convert to string or some bs
        let str: f64 = match str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid");
                continue;
            }
        };

        // Yay!
        println!("Yay! Your number plus 23 is equal to {}.", str + 23.9999);
    }
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
