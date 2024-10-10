use super::super::util::math;
use rand::prelude::*;
pub struct Client {
    pub name: String,
    seed: u32,
    pub modulo: u32,
    pub secret: u32,
    key: u32,
}

impl Client {
    pub fn new(name: &str, modulo: &u32, base: &u32) -> Client {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();
        Client {
            name: name.to_string(),
            seed: seed,
            modulo: *modulo,
            secret: math::pow_by_montgomery_ladder(&base, &seed, &modulo),
            key: 0,
        }
    }
    /**
     * 他のクライアントと鍵交換を行う。
     * このメソッドを呼び出すと、自身の鍵が更新される。
     *
     */
    pub fn exhange_key(&mut self, other: &mut Client) {
        self.key = math::pow_by_montgomery_ladder(&other.secret, &self.seed, &self.modulo);
        other.key = math::pow_by_montgomery_ladder(&self.secret, &other.seed, &self.modulo);
    }

    /**
     * Show the key of the client
     */
    pub fn show_my_key(&self) {
        println!("My Key is {}", self.key);
    }
}
