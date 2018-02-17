#[derive(PartialEq)]
#[derive(Debug)]
pub enum State {
    LeftShift,
    RightShift,
    Increment,
    Decrement,
    Read,
    Write,
    RightLoop,
    LeftLoop,
}

pub fn tokenize(token: &str) -> Option<State> {
    match token {
        "<" => Some(State::LeftShift),
        ">" => Some(State::RightShift),
        "+" => Some(State::Increment),
        "-" => Some(State::Decrement),
        "," => Some(State::Read),
        "." => Some(State::Write),
        "[" => Some(State::RightLoop),
        "]" => Some(State::LeftLoop),
        _ => None
    }
}