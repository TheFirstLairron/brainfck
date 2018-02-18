extern crate rustfuck;
use rustfuck::tape::{get_command, Tape};

fn main() {
    let mut tape = Tape::new();
    tape.process_token(get_command(",").unwrap());
    println!("{}", *tape.value_at_index(tape.current_index()).unwrap());
    tape.process_token(get_command("+").unwrap());
    println!("{}", *tape.value_at_index(tape.current_index()).unwrap());
    tape.process_token(get_command(",").unwrap());
    println!("{}", *tape.value_at_index(tape.current_index()).unwrap());
    tape.process_token(get_command("+").unwrap());
    println!("{}", *tape.value_at_index(tape.current_index()).unwrap());
}
