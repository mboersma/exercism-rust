//! Functions for Diffie-Hellman key exchange.

use rand::Rng;

// Returns a random private key larger than one but less than the number specified.
pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2, p)
}

// Calculates the public key value.
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g, a, p)
}

// Calculates the secret value.
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub, a, p)
}

// Calculates a number raised to a power, based on modular exponentiation to
// overcome Rust's limits on `pow()`.
// Copied from Robb Cobb's blog post, because it explains the algorithm so well
// and I could not possibly implement it better.
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}
