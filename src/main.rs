struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    fn holds(&self, o: &Rectangle) -> bool {
        self.w > o.w && self.h > o.h
    }

    // initializer for new type or something??
    fn square(size: u32) -> Self {
        Self {
            w: size,
            h: size,
        }
    }
}

fn main() {
    let sqr = Rectangle::square(100);

    println!("w: {}, h: {}", sqr.w, sqr.h);
}

// https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator
