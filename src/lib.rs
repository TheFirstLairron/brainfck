mod tests {

// TODO: Extract to its own file
#[derive(PartialEq)]
#[derive(Debug)]
enum Token {
    LeftShift,
    RightShift,
    Increment,
    Decrement,
    Read,
    Write,
    RightLoop,
    LeftLoop,
}

// TODO: Extract to proper file
fn tokenize(token: &str) -> Token {
    match token {
        "<" => Token::LeftShift,
        ">" => Token::RightShift,
        "+" => Token::Increment,
        "-" => Token::Decrement,
        "," => Token::Read,
        "." => Token::Write,
        "[" => Token::RightLoop,
        "]" => Token::LeftLoop,

        // Is there something better to return as default?
        _ => Token::LeftShift
    }
}

#[cfg(test)]
    #[test]
    fn tokenize_leftshift(){
        assert_eq!(tokenize("<"), Token::LeftShift);
    }
    
    #[test]
    fn tokenize_rightshift(){
        assert_eq!(tokenize(">"), Token::RightShift);
    }

    #[test]
    fn tokenize_increment(){
        assert_eq!(tokenize("+"), Token::Increment);
    }

    #[test]
    fn tokenize_decrement(){
        assert_eq!(tokenize("-"), Token::Decrement);
    }

    #[test]
    fn tokenize_read(){
        assert_eq!(tokenize(","), Token::Read);
    }

    #[test]
    fn tokenize_write(){
        assert_eq!(tokenize("."), Token::Write);
    }

    #[test]
    fn tokenize_rightloop(){
        assert_eq!(tokenize("["), Token::RightLoop);
    }

    #[test]
    fn tokenize_leftloop(){
        assert_eq!(tokenize("]"), Token::LeftLoop);
    }
}
