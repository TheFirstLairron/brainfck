use interpreter::Command;

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
        match token {
            Command::Increment => Result::Ok(()),
            Command::Decrement => Result::Ok(()),
            Command::LeftShift => {
                if self.current_index() == 0 {
                    Result::Err(Failure::MemoryUnderstep)
                } else {
                    self.pointer -= 1;
                    Result::Ok(())
                }
            }
            Command::RightShift => {
                if self.pointer == self.cells.len() {
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
mod tape_tests {
    use super::*;

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
}
