/// Verify that pairs of brackets, braces, and parentheses match and nest
/// properly in a given string.
///
/// # Examples
///
/// ```
/// let s = "[if(contains(split('{canadacentral},{centralus}',','),variables('location')),3,if(equals('{centraluseuap}',variables('location')),1,2))]";
/// assert!(matching_brackets::brackets_are_balanced(s));
/// ```
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = String::new();

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => stack.push(c),
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => continue,
        }
    }

    stack.is_empty()
}
