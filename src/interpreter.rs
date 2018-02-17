#[derive(PartialEq, Debug)]
pub enum Command {
    LeftShift,
    RightShift,
    Increment,
    Decrement,
    Read,
    Write,
    RightLoop,
    LeftLoop,
}

pub fn tokenize(token: &str) -> Option<Command> {
    match token {
        "<" => Some(Command::LeftShift),
        ">" => Some(Command::RightShift),
        "+" => Some(Command::Increment),
        "-" => Some(Command::Decrement),
        "," => Some(Command::Read),
        "." => Some(Command::Write),
        "[" => Some(Command::RightLoop),
        "]" => Some(Command::LeftLoop),
        _ => None,
    }
}
