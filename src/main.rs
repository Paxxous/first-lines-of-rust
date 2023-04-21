struct YourSelf {
    name: String,
    level: u64,
}

impl YourSelf {
    fn improve(&mut self) {
        self.level = self.level + 1;
    }
}

fn main() {
    println!("Hello world");

    let mut you = YourSelf {
        name: String::from("haba"),
        level: 0,
    };

    for _ in 1..=1000000000 {
        you.improve();
    }

    println!("Now {} is at level {}", you.name, you.level);
}

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
