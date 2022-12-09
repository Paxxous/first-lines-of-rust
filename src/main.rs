use std::io;

fn main() {
    let a = [1, 2, 3, 6, 7, 88];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("PLease input a valid string");

    // use usize when indexing some form of collection, (such as an array)
    let index: usize = index
        .trim()
        .parse()
        .expect("Please input a number");

    let element = a[index];

    println!("{}", element);
}

// https://doc.rust-lang.org/book/ch03-02-data-types.html
