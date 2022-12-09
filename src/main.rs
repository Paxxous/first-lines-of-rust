fn main() {
    let x = literally_just_returns_five();
    println!("{x}"); // WOAHHHHHH
}

/*
 * Bro I already know how functions work.
 * Look at me with these fancy comments
 * I've been only using these for adding insight
 * to what I am doing.
 */

fn literally_just_returns_five() -> i32 {
    let x = 99;
    x + 1
}

// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions
