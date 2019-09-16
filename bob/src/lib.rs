//! Reply to conversation as Bob, the lackadaisical teenager.

const CALM_DOWN: &str = "Calm down, I know what I'm doing!";
const WHOA_CHILL: &str = "Whoa, chill out!";
const SURE: &str = "Sure.";
const FINE: &str = "Fine. Be that way!";
const WHATEVER: &str = "Whatever.";

/// Return Bob's response to a statement, a question, or yelling.
pub fn reply(message: &str) -> &str {
    let msg: &str = message.trim();

    match msg.is_empty() {
        true => FINE,
        false => {
            let is_question: bool = msg.ends_with("?");
            let contains_alphabetic_characters: bool =
                msg.chars().any(|character| character.is_alphabetic());
            let is_uppercase: bool = contains_alphabetic_characters && msg == msg.to_uppercase();
            let is_yelled: bool = contains_alphabetic_characters && is_uppercase;

            match (is_yelled, is_question) {
                (true, true) => CALM_DOWN,
                (true, false) => WHOA_CHILL,
                (false, true) => SURE,
                (false, false) => WHATEVER,
            }
        }
    }
}
