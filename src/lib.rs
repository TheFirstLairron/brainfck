mod interpreter;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenize_leftshift(){
        assert_eq!(interpreter::tokenize("<"), Some(interpreter::State::LeftShift));
    }
    
    #[test]
    fn tokenize_rightshift(){
        assert_eq!(interpreter::tokenize(">"), Some(interpreter::State::RightShift));
    }

    #[test]
    fn tokenize_increment(){
        assert_eq!(interpreter::tokenize("+"), Some(interpreter::State::Increment));
    }

    #[test]
    fn tokenize_decrement(){
        assert_eq!(interpreter::tokenize("-"), Some(interpreter::State::Decrement));
    }

    #[test]
    fn tokenize_read(){
        assert_eq!(interpreter::tokenize(","), Some(interpreter::State::Read));
    }

    #[test]
    fn tokenize_write(){
        assert_eq!(interpreter::tokenize("."), Some(interpreter::State::Write));
    }

    #[test]
    fn tokenize_rightloop(){
        assert_eq!(interpreter::tokenize("["), Some(interpreter::State::RightLoop));
    }

    #[test]
    fn tokenize_leftloop(){
        assert_eq!(interpreter::tokenize("]"), Some(interpreter::State::LeftLoop));
    }

    #[test]
    fn tokenize_badcharacter(){
        assert_eq!(interpreter::tokenize("o"), None);
    }
}
