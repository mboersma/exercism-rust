/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars().all(|c| c.is_digit(10) || c.is_whitespace())
        && code.chars().filter(|c| c.is_digit(10)).count() > 1
        && code
            .chars()
            .filter(|c| c.is_digit(10))
            .rev()
            .enumerate()
            .map(|(i, c)| {
                let mut n = c.to_digit(10).unwrap();
                if i % 2 == 1 {
                    n *= 2;
                    if n > 9 {
                        n -= 9;
                    }
                }
                n
            })
            .sum::<u32>()
            % 10
            == 0
}
