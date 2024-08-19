/// Struct representing the plugboard.
/// This translates characters through
pub struct Plugboard {
    wires: [u8; 26],
}

impl Plugboard {
    /// Create a new plugboard instance, with no wires added
    pub fn new() -> Plugboard {
        let mut wires: [u8; 26] = [0; 26];
        for i in 0..26 {
            wires[i] = i as u8;
        }
        Plugboard {
            wires
        }
    }

    /// Add a wire between the start and end char. The chars
    /// can be upper or lowercase.
    pub fn add_wire(&mut self, start: char, end: char) -> Result<(),Err(PlugboardError)> {
        let start = start.to_ascii_lowercase() as u8 - 97u8;
        let end = end.to_ascii_lowercase() as u8 - 97u8;
        // Check if there is already a wire originating from one of the ends
        if self.wires[start as usize] != start || self.wires[end as usize] !=end{
            return Err(PlugboardError::OverlappingWires)
        }
        // If they are the same, just skip this
        if start == end {
            return Ok(());
        }
        // Set the start and end to each other
        self.wires[start as usize] = end;
        self.wires[end as usize] = start;
        Ok(())
    }

    /// Remove a wire from the plugboard which starts at start, and ends at end
    /// (direction unimportant, so a-e and e-a are equivalent)
    pub fn remove_wire(&mut self, start: char, end:char)->Result<(),Err(PlugboardError)>{
        let start = start.to_ascii_lowercase() as u8 - 97u8;
        let end = end.to_ascii_lowercase() as u8 - 97u8;
        if self.wires[start as usize] == end && self.wires[end as usize]==start{
            self.wires[start as usize] = start;
            self.wires[end as usize] = end;
            return Ok(())
        }
        Err(PlugboardError::WireDoesntExist)
    }

    /// Take in a char and translate it through the plugboard
    pub fn translate_char(&self, input_char: char) -> char {
        let input_val = input_char.to_ascii_lowercase() as u8 - 97u8;
        (self.translate_u8(input_val) + 97u8) as char
    }

    /// Take in u8 representing a char and translate it through the plugboard
    fn translate_u8(&self, input_val: u8) -> u8 {
        self.wires[input_val as usize]
    }
}

pub enum PlugboardError{
    WireDoesntExist,
    OverlappingWires,
}

#[cfg(test)]
mod test_plugboard {
    use crate::machine::plugboard::Plugboard;

    #[test]
    fn test_translate(){
        let mut test_board = Plugboard::new();
        test_board.add_wire('b', 'z');
        assert_eq!('a', test_board.translate_char('a'));
        assert_eq!(0u8, test_board.translate_u8(0u8));
        assert_eq!('b', test_board.translate_char('z'));
        assert_eq!('z', test_board.translate_char('b'));
    }
}