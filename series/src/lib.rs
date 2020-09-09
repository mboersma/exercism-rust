//! series finds contiguous substrings in a string.

// Returns a vector of substrings with the given length.
pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut subs = vec![];
    if len <= digits.len() {
        let end = digits.len() - len;
        for i in 0..=end {
            subs.push(digits[i..i + len].to_string());
        }
    }
    subs
}
