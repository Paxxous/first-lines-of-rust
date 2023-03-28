#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // godamina
        self.w * self.h
    }
}

fn main() {
    let re = Rectangle {
        w: 10,
        h: 20
    };

    println!("Fucntion {:?} and {}", re, re.area());
}

// https://doc.rust-lang.org/book/ch05-02-example-structs.html
