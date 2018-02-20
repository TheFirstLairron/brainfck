use super::*;

#[test]
fn test_data_storage() {
    const VALUE: u8 = 200;
    let mut tape = Tape::new();

    tape.store_byte(VALUE);
    assert_eq!(*tape.value_at_index(tape.current_index()).unwrap(), VALUE);
}

#[test]
fn test_increment() {
    // arrange
    const RESULT: u8 = 1;
    let mut tape = Tape::new();

    // act
    tape.process_token(Command::Increment);

    // assert
    assert_eq!(RESULT, *tape.value_at_index(tape.current_index()).unwrap());
}

#[test]
fn test_decrement() {
    // arrange
    const RESULT: u8 = 255;
    let mut tape = Tape::new();

    // act
    tape.process_token(Command::Decrement);

    // assert
    assert_eq!(RESULT, *tape.value_at_index(tape.current_index()).unwrap());
}

#[test]
fn right_shift_success() {
    const RESULT: usize = 1;
    let mut tape = Tape::new();

    tape.process_token(Command::RightShift);

    assert_eq!(RESULT, tape.current_index());
}

#[test]
fn left_shift_success() {
    const RESULT: usize = 0;
    let mut tape = Tape::new();

    tape.process_token(Command::RightShift);
    tape.process_token(Command::LeftShift);

    assert_eq!(RESULT, tape.current_index());
}

#[test]
fn left_shift_failure() {
    let mut tape = Tape::new();

    assert!(tape.process_token(Command::LeftShift).is_err());
}

#[test]
fn get_command_leftshift() {
    assert_eq!(get_command("<"), Some(Command::LeftShift));
}

#[test]
fn get_command_rightshift() {
    assert_eq!(get_command(">"), Some(Command::RightShift));
}

#[test]
fn get_command_increment() {
    assert_eq!(get_command("+"), Some(Command::Increment));
}

#[test]
fn get_command_decrement() {
    assert_eq!(get_command("-"), Some(Command::Decrement));
}

#[test]
fn get_command_read() {
    assert_eq!(get_command(","), Some(Command::Read));
}

#[test]
fn get_command_write() {
    assert_eq!(get_command("."), Some(Command::Write));
}

#[test]
fn get_command_rightloop() {
    assert_eq!(get_command("["), Some(Command::RightLoop));
}

#[test]
fn get_command_leftloop() {
    assert_eq!(get_command("]"), Some(Command::LeftLoop));
}

#[test]
fn get_command_badcharacter() {
    assert_eq!(get_command("o"), None);
}
