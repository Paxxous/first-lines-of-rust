fn main() {
    let mut count = 0;
    'count_up: /* Label your loops tf tf? */ loop {
        println!("Count: {}", count);

        let mut remaining = 0;
        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'count_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("Count ends at {}", count);
}

// https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
