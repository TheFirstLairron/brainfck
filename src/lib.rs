mod interpreter;
mod tape;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_leftshift() {
        assert_eq!(
            interpreter::tokenize("<"),
            Some(interpreter::Command::LeftShift)
        );
    }

    #[test]
    fn tokenize_rightshift() {
        assert_eq!(
            interpreter::tokenize(">"),
            Some(interpreter::Command::RightShift)
        );
    }

    #[test]
    fn tokenize_increment() {
        assert_eq!(
            interpreter::tokenize("+"),
            Some(interpreter::Command::Increment)
        );
    }

    #[test]
    fn tokenize_decrement() {
        assert_eq!(
            interpreter::tokenize("-"),
            Some(interpreter::Command::Decrement)
        );
    }

    #[test]
    fn tokenize_read() {
        assert_eq!(interpreter::tokenize(","), Some(interpreter::Command::Read));
    }

    #[test]
    fn tokenize_write() {
        assert_eq!(
            interpreter::tokenize("."),
            Some(interpreter::Command::Write)
        );
    }

    #[test]
    fn tokenize_rightloop() {
        assert_eq!(
            interpreter::tokenize("["),
            Some(interpreter::Command::RightLoop)
        );
    }

    #[test]
    fn tokenize_leftloop() {
        assert_eq!(
            interpreter::tokenize("]"),
            Some(interpreter::Command::LeftLoop)
        );
    }

    #[test]
    fn tokenize_badcharacter() {
        assert_eq!(interpreter::tokenize("o"), None);
    }
}
