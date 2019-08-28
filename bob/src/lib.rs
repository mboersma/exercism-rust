//! Reply to conversation as Bob, the lackadaisical teenager.

/// Return Bob's response to a statement, a question, or yelling.
pub fn reply(message: &str) -> &str {
    let m = message.trim();
    // Does it have any letters and are they in ALL CAPS?
    if m.chars().any(|c| c.is_alphabetic()) && m == m.to_uppercase() {
        if m.ends_with("?") {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    } else if m.ends_with("?") {
        return "Sure.";
    } else if m == "" {
        return "Fine. Be that way!";
    }
    "Whatever."
}
