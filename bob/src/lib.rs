//! Reply to conversation as Bob, the lackadaisical teenager.

const CALM_DOWN: &str = "Calm down, I know what I'm doing!";
const WHOA_CHILL: &str = "Whoa, chill out!";
const SURE: &str = "Sure.";
const FINE: &str = "Fine. Be that way!";
const WHATEVER: &str = "Whatever.";

/// Return Bob's response to a statement, a question, or yelling.
pub fn reply(message: &str) -> &str {
    let m = message.trim();
    // Does it have any letters and are they in ALL CAPS?
    if m.chars().any(|c| c.is_alphabetic()) && m == m.to_uppercase() {
        if m.ends_with("?") {
            return CALM_DOWN;
        } else {
            return WHOA_CHILL;
        }
    } else if m.ends_with("?") {
        return SURE;
    } else if m == "" {
        return FINE;
    }
    WHATEVER
}
