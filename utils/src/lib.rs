pub mod log;
pub mod mail;
pub mod version;

use rand::Rng;

pub fn random_in_range(num: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..num)
}
