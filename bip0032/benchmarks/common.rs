use rand::{Rng, RngCore};

pub fn random_seed() -> Vec<u8> {
    let mut rng = rand::rng();
    // Some bip32 libraries only accept 16/32/64-byte seeds, so use that subset here.
    let len = match rng.random_range(0..=2) {
        0 => 16,
        1 => 32,
        _ => 64,
    };
    let mut seed = vec![0u8; len];
    rng.fill_bytes(&mut seed);
    seed
}
