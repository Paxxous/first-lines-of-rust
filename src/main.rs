// the main function 
fn main() {
    let x = 5;

    let x = x + 5;

    {
        let x = x * 2;
        println!("X in this scope equals {x}");
    }

    println!("X in this larger scope equals {x}");
}

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#handling-invalid-input
