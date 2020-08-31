/// Checks if a number is an "Armstrong number": a number that is the sum of its
/// own digits each raised to the power of the number of digits.
///
/// # Examples
///
/// ```
/// assert!(armstrong_numbers::is_armstrong_number(153));
/// assert!(!armstrong_numbers::is_armstrong_number(154));
/// ```
pub fn is_armstrong_number(num: u32) -> bool {
    let s = num.to_string();
    let exp = s.len() as u32;
    num == s.chars()
        .filter_map(|n| n.to_digit(10))
        .map(|n| (n as u32).pow(exp))
        .sum()
}
