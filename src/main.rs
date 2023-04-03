// woah enum more useful than I've expected.
enum ipkind {
    V4,
    V6,
}

struct internet {
    // so it has to be used in function then.
    ipv: ipkind,
    address: String,
    on: bool,
}

// Why I created this one 
fn route(ip: ipkind) {

}

fn main() {
    // We created it.
    let int = internet {
        ipv: ipkind::V4 /* the best one */,
        address: String::from("1.0.0.1"),
        on: true,
   };
}

// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
