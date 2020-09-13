//! Calculates the nth prime starting from zero.

// All credit for this solution goes to plippe at exercism. After experimentation,
// I threw out my old PrimeCandidates approach for something more functional and
// came up with a horrible version of this, but then I saw his solution, and...bravo!

// Returns true if n is a prime number.
pub fn is_prime(n: u32) -> bool {
    !(2..n - 1).any(|i| n % i == 0)
}

// Returns the nth prime starting from zero.
pub fn nth(n: u32) -> u32 {
    (2..).filter(|c| is_prime(*c)).nth(n as usize).unwrap()
}
