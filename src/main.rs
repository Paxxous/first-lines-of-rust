use std::io;

#[derive(Debug)]
struct Internet {
    service: String,
    ip: String,
    ping: u32,
}

// work?
enum Terminal {}

impl Terminal {
    fn grab(prompt: String) -> String {
        println!("{}", prompt);

        let mut str = String::new();
        io::stdin()
            .read_line(&mut str)
            .expect("Failed to read line");

        str
    }
}

fn main() {
    println!("Hello there my good friend ;)\nWe're nearly done setting up your internet");
    
    let user_service = Terminal::grab(
        String::from("What service would you like?")
    );

    println!("Thank you for the input ;)");

    let this_fuckers_internet = Internet {
        service: user_service,
        ip: String::from("69.420.6.9"),
        ping: 69420, // in ms
    };

    println!("Here's some data ab our internet mg: {:#?}\n(ping is in ms)", this_fuckers_internet);
}

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
