fn main() {
    let y = {
        let x = 3;
        x + 1 // Won't return a value??? Semicolons means that it's a statement??
        // WAIT HOLY SHIT THAT'S HOW YOU RETURN VALUE IN FUNCTION
    };

    println!("The value of y is {y}");
}

// https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#statements-and-expressions
