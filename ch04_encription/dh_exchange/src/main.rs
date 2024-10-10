mod diffle_hellemann;

use diffle_hellemann::client::Client;

fn main() {
    let c = Client::new("Alice");
    c.hello();
}
