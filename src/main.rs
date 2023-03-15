struct Client {
    connected: bool,
    name: String,
    times_logged_in: u64,
}

fn build_client(
        connected: bool,
        name: String,
        times_logged_in: u64
    ) -> Client {

    Client {
        connected,
        name,
        times_logged_in,
    }
}

fn main() {
    let client = build_client(
        true,
        String::from("the"),
        1
    );

    println!("client online: {} client name: {}, times logged in: {}",
        client.connected,
        client.name,
        client.times_logged_in,
    );
}

// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html
