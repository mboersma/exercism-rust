pub fn reply(message: &str) -> &str {
    if message == message.to_uppercase() {
        if message.ends_with("?") {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Whoa, chill out!";
        }
    } else if message.ends_with("?") {
        return "Sure.";
    }
    "Whatever."
}
