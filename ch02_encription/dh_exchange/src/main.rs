mod diffle_hellemann;
mod util;

use diffle_hellemann::client::Client;

fn main() {
    let p = 3820;
    let g = 4574;
    let mut alice = Client::new("Alice", &p, &g);
    let mut bob = Client::new("Bob", &p, &g);

    alice.exhange_key(&mut bob);

    alice.show_my_key();
    bob.show_my_key();
}
