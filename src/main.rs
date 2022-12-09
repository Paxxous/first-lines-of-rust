// the main function 
fn main() {
    // Tuples woah!
    // We can declare this explicitly :D
    let tupe = (10, 5.25, 68.01, 0xff);
    let (_a, _b, _c, d) = tupe;

    println!("{d}");
}

// https://doc.rust-lang.org/book/ch03-02-data-types.html

// https://en.wikipedia.org/wiki/Software_bug#/media/File:First_Computer_Bug,_1945.jpg <- lmao
