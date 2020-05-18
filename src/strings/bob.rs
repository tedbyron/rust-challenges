//! given a message, return Bob's lazy responses to questions, yelling, yelling
//! questions, silence, or anything else

#[allow(dead_code)]
pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let upper = message.contains(char::is_alphabetic) && message.to_ascii_uppercase() == message;

    match message {
        m if m.ends_with('?') && upper => "Calm down, I know what I'm doing!",
        m if m.ends_with('?') => "Sure.",
        _ if upper => "Whoa, chill out!",
        m if m.is_empty() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
