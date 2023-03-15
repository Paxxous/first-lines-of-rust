struct Client {
    connected: bool,
    name: String,
}

fn main() {
    let client = Client {
        connected: true,
        name: String::from("poopshartbuttholegayman69420"),
    };

    println!("client online: {}, client name: {}",
        client.connected,
        client.name);
}

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
