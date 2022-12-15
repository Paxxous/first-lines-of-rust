fn main() {
    let mut count = 32;
    while count != 0 {
        count -= 1;
        println!("{}", count);
    }

    println!("There's the shitter: {}", count);
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
