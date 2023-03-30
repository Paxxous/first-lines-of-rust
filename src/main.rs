#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        // huuh?? oh you can add logic :D
        self.w > 0
    }

    fn holds(&self, o: &Rectangle) -> bool {
        self.w > o.w && self.h > o.h
    }
}

fn main() {
    let re = Rectangle {
        w: 10,
        h: 20
    };

    let is = Rectangle {
        w: 230498,
        h: 8902598,
    };

    if is.holds(&re) {
        println!("It's too FAT.")
    }
}

// https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator
