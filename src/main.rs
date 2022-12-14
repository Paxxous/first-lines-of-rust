use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    
    loop {
        // Merry Easter!!!!
        println!("Look I'm copying you! {}", buffer);
    }
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops
