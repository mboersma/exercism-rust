/// Computes the prime factors of a given natural number.
///
/// # Examples
///
/// ```
/// assert_eq!(prime_factors::factors(147), vec![3, 7, 7]);
/// ```
pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut n = n;
    let mut d = 2;
    while d <= n {
        if n % d == 0 {
            factors.push(d);
            n /= d;
        } else {
            d += 1;
        }
    }
    factors
}
