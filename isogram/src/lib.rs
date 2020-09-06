//! Determine if a word or phrase is an isogram: one which repeats no letter.

use std::collections::HashSet;

/// Checks a string to see if it is an isogram.
///
/// # Examples
///
/// ```
/// assert!(isogram::check("ten"));
/// assert!(!isogram::check("eleven"));
/// ```
pub fn check(candidate: &str) -> bool {
    let mut unique = HashSet::new();
    candidate
        .to_ascii_lowercase()
        .chars()
        .all(move |c| match c {
            '-' | ' ' => true,
            _ => unique.insert(c),
        })
}
