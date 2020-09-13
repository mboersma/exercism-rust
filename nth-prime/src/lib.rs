//! Given a number n, determine what the nth prime is.
/// All credit for this solution goes to plippe at exercism!

/// Returns the nth prime in the series 2, 3, 5, 7, 11, 13, ...
///
/// # Examples
///
/// ```
/// assert_eq!(nth_prime::nth(5), 13);
/// ```
pub fn nth(n: u32) -> u32 {
    (2..).filter(|c| is_prime(*c)).nth(n as usize).unwrap()
}

/// Returns true if n is a prime number.
pub fn is_prime(n: u32) -> bool {
    !(2..n / 2 + 1).any(|i| n % i == 0)
}
