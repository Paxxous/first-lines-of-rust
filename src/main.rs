const COUNT_UNTIL: i64 = 4123489999;
fn main() {
    let mut counter = 0;
    let end = loop {
        counter += 1;

        if counter == COUNT_UNTIL {
            break counter;
        }
    };

    println!("ended with result of {}", end);
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html#repetition-with-loops
