//! Reply to conversation as Bob, the lackadaisical teenager.

const CALM_DOWN: &str = "Calm down, I know what I'm doing!";
const WHOA_CHILL: &str = "Whoa, chill out!";
const SURE: &str = "Sure.";
const FINE: &str = "Fine. Be that way!";
const WHATEVER: &str = "Whatever.";

/// Return Bob's response to a statement, a question, or yelling.
pub fn reply(message: &str) -> &str {
    let m: &str = message.trim();
    let is_question: bool = m.ends_with("?");
    let contains_alphabetic_characters: bool = m.chars().any(|character| character.is_alphabetic());
    let is_uppercase: bool = contains_alphabetic_characters && m == m.to_uppercase();
    let is_yelled: bool = contains_alphabetic_characters && is_uppercase;

    if is_yelled {
        if is_question {
            return CALM_DOWN;
        } else {
            return WHOA_CHILL;
        }
    } else if is_question {
        return SURE;
    } else if m.is_empty() {
        return FINE;
    }
    WHATEVER
}
