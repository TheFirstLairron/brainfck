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

pub fn get_command(token: &str) -> Option<Command> {
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

pub enum Failure {
    MemoryOverstep,
    ReadFailure,
    WriteFailure,
    MemoryUnderstep,
}

pub struct Tape {
    cells: [u8; 100],
    pointer: usize,
}

impl Tape {
    /// Create a new instance of the Tape
    ///
    /// # Panics
    ///
    /// None
    pub fn new() -> Tape {
        Tape {
            // Default to an array of 100 cells with the default value 0.
            cells: [0; 100],
            // Default to first cell in the tape
            pointer: 0,
        }
    }

    pub fn current_index(&self) -> usize {
        self.pointer
    }

    pub fn value_at_index(&self, index: usize) -> Option<&u8> {
        self.cells.get(index)
    }

    pub fn process_token(&mut self, token: Command) -> Result<(), Failure> {
        const MAX_VALUE: u8 = 255;
        const MIN_VALUE: u8 = 0;
        match token {
            Command::Increment => {
                let mut value = self.cells[self.pointer];
                if value == MAX_VALUE {
                    value = MIN_VALUE;
                } else {
                    value += 1;
                }
                self.cells[self.pointer] = value;
                Result::Ok(())
            }
            Command::Decrement => {
                let mut value = self.cells[self.pointer];
                if value == MIN_VALUE {
                    value = MAX_VALUE;
                } else {
                    value -= 1;
                }
                self.cells[self.pointer] = value;
                Result::Ok(())
            }
            Command::LeftShift => {
                if self.current_index() == 0 {
                    Result::Err(Failure::MemoryUnderstep)
                } else {
                    self.pointer -= 1;
                    Result::Ok(())
                }
            }
            Command::RightShift => {
                if self.pointer == self.cells.len() - 1 {
                    Result::Err(Failure::MemoryOverstep)
                } else {
                    self.pointer += 1;
                    Result::Ok(())
                }
            }
            Command::Read => Result::Ok(()),
            Command::Write => Result::Ok(()),
            Command::LeftLoop => Result::Ok(()),
            Command::RightLoop => Result::Ok(()),
        }
    }
}

#[cfg(test)]
mod test;
