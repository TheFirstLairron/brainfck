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
    fn new() -> Tape {
        Tape {
            // Default to an array of 100 cells with the default value 0. 
            cells: [0; 100],
            // Default to first cell in the tape
            pointer: 0,
        }
    }
}
