mod client;
mod server;

use std::thread;

fn main() {
    // Your code here
    thread::spawn(|| {
        server::server();
    });
    client::client();
}
