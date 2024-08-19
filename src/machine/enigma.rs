use crate::machine::plugboard;
use crate::machine::reflector;
use crate::machine::rotor;
use std::fmt;

/// Struct representing the Enigma Machine
#[derive(Clone)]
pub struct Enigma {
    /// Rotors placed in the machine
    rotors: [rotor::Rotor; 3],
    /// Plugboard configuration for the machine
    plugboard: plugboard::Plugboard,
    /// Reflector in the machine
    reflector: reflector::Reflector,
}

impl fmt::Display for Enigma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Rotor Configuration:
\tRotor 1:{}
\tRotor 2:{}
\tRotor 3:{}
Reflector Configuration:
\t{}
Plugboard Configuration:
\t{}
",
               self.rotors[0], self.rotors[1], self.rotors[2],
               self.reflector,
               self.plugboard
        )
    }
}

impl Enigma {
    /// Create a new enigma machine with the provided rotors, plugboard, and reflector
    pub(crate) fn new(r1: rotor::Rotor, r2: rotor::Rotor, r3: rotor::Rotor,
               plugboard: plugboard::Plugboard, reflector: reflector::Reflector) -> Self {
        let rotors: [rotor::Rotor; 3] = [r1, r2, r3];
        Self {
            rotors,
            plugboard,
            reflector,
        }
    }

    /// Create a default enigma configuration with rotors I,II,III and reflector A, with
    /// no wires in the plugboard
    pub fn default() -> Self {
        Self::new(rotor::Rotor::new_i(), rotor::Rotor::new_ii(), rotor::Rotor::new_iii(),
                  plugboard::Plugboard::new(),  reflector::Reflector::new_a())
    }

    /// Set a rotor in a given position
    pub fn choose_rotor(&mut self, rotor: &str, position: u8) -> Result<(), EnigmaError> {
        match rotor {
            "I" => {
                self.rotors[position as usize] = rotor::Rotor::new_i();
                Ok(())
            }
            "II" => {
                self.rotors[position as usize] = rotor::Rotor::new_ii();
                Ok(())
            }
            "III" => {
                self.rotors[position as usize] = rotor::Rotor::new_iii();
                Ok(())
            }
            "IV" => {
                self.rotors[position as usize] = rotor::Rotor::new_iv();
                Ok(())
            }
            "V" => {
                self.rotors[position as usize] = rotor::Rotor::new_v();
                Ok(())
            }
            "VI" => {
                self.rotors[position as usize] = rotor::Rotor::new_vi();
                Ok(())
            }
            "VII" => {
                self.rotors[position as usize] = rotor::Rotor::new_vii();
                Ok(())
            }
            "VIII" => {
                self.rotors[position as usize] = rotor::Rotor::new_viii();
                Ok(())
            }
            r => {
                Err(EnigmaError::InvalidRotor(r.to_string()))
            }
        }
    }

    /// Set a particular reflector
    pub fn choose_reflector(&mut self, reflector: &str) -> Result<(), EnigmaError> {
        match reflector {
            "a" | "A" => {
                self.reflector = reflector::Reflector::new_a();
                Ok(())
            }
            "b" | "B" => {
                self.reflector = reflector::Reflector::new_b();
                Ok(())
            }
            "c" | "C" => {
                self.reflector = reflector::Reflector::new_c();
                Ok(())
            }
            r => {
                Err(EnigmaError::InvalidReflector(r.to_string()))
            }
        }
    }

    /// Change the rotor settings
    pub fn set_rotors(&mut self, setting: &str) -> Result<(), EnigmaError> {
        for (idx, c) in setting.chars().enumerate() {
            if idx < 3 {
                self.rotors[idx].set(c);
            } else { break; };
        }
        Ok(())
    }

    /// Change the reflector setting
    pub fn set_reflector(&mut self, setting: &str) -> Result<(), EnigmaError> {
        for (idx, c) in setting.chars().enumerate() {
            if idx < 1 {
                self.reflector.set(c);
            } else {
                break;
            }
        }
        Ok(())
    }

    /// Add a single wire to the plugboard, should be a string of the form "<start>-<end>",
    /// for example "a-e" connects a and e on the plugboard
    pub fn add_plugboard_wire(&mut self, wire: &str) -> Result<(), EnigmaError> {
        let mut letters: Vec<char> = Vec::new();
        for s in wire.split("-") {
            if s.len() == 1 {
                letters.push(s.chars().next()
                    .expect("Error in converting length 1 str to char"))
            } else {
                return Err(EnigmaError::InvalidPlugboardWire(wire.to_string()));
            }
        }
        if letters.len() != 2 {
            return Err(EnigmaError::InvalidPlugboardWire(wire.to_string()));
        }
        match self.plugboard.add_wire(letters[0], letters[1]) {
            Ok(()) => { Ok(()) }
            Err(err) => {
                match err {
                    plugboard::PlugboardError::OverlappingWires => {
                        Err(EnigmaError::PlugboardWireOverlap(
                            format!("Couldn't add wire due to overlap {}", wire).to_string()))
                    }
                    _ => { panic!("Unknown error when inserting plugboard wire!") }
                }
            }
        }
    }

    /// Add a set of wires to the plugboard, should be a string of the form
    /// "wire,wire,wire,..." where each wire is a string of the form
    /// "<start>-<end>. For example, the string "a-e,b-d,x-z" will
    /// add connections between a and e; b and d; and x and z.
    pub fn add_plugboard_wires(&mut self, wires: &str) -> Result<(), EnigmaError> {
        for wire in wires.split(",") {
            self.add_plugboard_wire(wire.trim())?
        }
        Ok(())
    }

    /// Remove a wire from the plugboard by specifying one of its ends
    pub fn remove_plugboard_wire(&mut self, wire: char) -> Result<(), EnigmaError> {
        let end = self.plugboard.translate_char(wire);
        match self.plugboard.remove_wire(wire, end) {
            Ok(()) => Ok(()),
            Err(_) => {
                Err(EnigmaError::PlugboardWireRemoveFailure(
                    format!("Couldn't remove {}", wire).to_string()
                ))
            }
        }
    }

    /// Remove a series of wires from the plugboard, represented by a comma separated list of
    /// characters, where each character represents one end of a wire to remove.
    pub fn remove_plugboard_wires(&mut self, wire: &str)->Result<(), EnigmaError>{
        for w in wire.split(","){
            match w.trim().chars().next(){
                None => {continue;}
                Some(c) => {self.remove_plugboard_wire(c)?;}
            }
        }
        Ok(())
    }

    /// Translate a string through the Enigma machine
    pub fn translate(&mut self, input: &str) -> String {
        let mut translated_str: String = String::new();
        for c in input.chars() {
            match c {
                'a'..='z' => {
                    translated_str.push(self.translate_char(c));
                    // Only step if letter actually passing through the machine
                    self.step();
                }
                'A'..='Z' => {
                    translated_str.push(self.translate_char(c.to_ascii_lowercase()));
                    // Only step if letter actually passing through the machine
                    self.step();
                }
                other => { translated_str.push(other); }
            }
        }
        translated_str
    }

    /// Pass a single character through the engima machine
    fn translate_char(&self, input: char) -> char {
        let mut transfer_char: char = input;
        // Pass through plugboard
        transfer_char = self.plugboard.translate_char(transfer_char);
        // Forward pass through rotors
        for r in self.rotors.iter() {
            transfer_char = r.translate_forward(transfer_char);
        }
        // Through the reflector
        transfer_char = self.reflector.translate(transfer_char);
        // Reverse pass through the rotors
        for r in self.rotors.iter().rev() {
            transfer_char = r.translate_reverse(transfer_char);
        }
        // Back through the plugboard
        transfer_char = self.plugboard.translate_char(transfer_char);
        transfer_char
    }

    /// Step the rotors of the enigma machine
    fn step(&mut self) {
        let mut to_step: bool = true;
        for r in self.rotors.iter_mut().rev() {
            if to_step {
                to_step = r.step();
            }
        }
    }
}

pub enum EnigmaError {
    InvalidRotor(String),
    InvalidReflector(String),
    InvalidPlugboardWire(String),
    PlugboardWireOverlap(String),
    PlugboardWireRemoveFailure(String),
}

impl fmt::Display for EnigmaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnigmaError::InvalidRotor(s) => { write!(f, "Invalid Rotor: {}", s) }
            EnigmaError::InvalidReflector(s) => { write!(f, "Invalid Reflector: {}", s) }
            EnigmaError::InvalidPlugboardWire(s) => { write!(f, "Invalid Plugboard Wire: {}", s) }
            EnigmaError::PlugboardWireOverlap(s) => { write!(f, "Plugboard Wire Overlap: {}", s) }
            EnigmaError::PlugboardWireRemoveFailure(s) => { write!(f, "Plugboard Wire Removal Failure: {}", s) }
        }
    }
}

#[cfg(test)]
mod test_enigma {
    use crate::machine::enigma::Enigma;

    #[test]
    fn test_translation() {
        let mut test_encoder = Enigma::default();
        _ = test_encoder.choose_reflector("B");
        let encoded = test_encoder.translate("aaaa");
        let mut test_decoder = Enigma::default();
        _ = test_decoder.choose_reflector("B");
        let decoded = test_decoder.translate(&encoded);
        assert_ne!(encoded, decoded);
        assert_ne!("aaaa", encoded);
        assert_eq!("aaaa", decoded);
        assert_eq!("nqxh", encoded);
    }

    #[test]
    fn test_plugboard() {
        let mut test_encoder = Enigma::default();
        _ = test_encoder.choose_reflector("C");
        _ = test_encoder.add_plugboard_wires("a-e,b-c,x-z,m-n");

        let mut test_decoder = test_encoder.clone();

        let encoded = test_encoder.translate("this is a coded message");
        let decoded = test_decoder.translate(&encoded);

        assert_eq!("this is a coded message", decoded);
        assert_ne!(encoded, "this is a coded message");
    }

    #[test]
    fn test_long_translation(){
        let mut test_encoder  = Enigma::default();
        _ = test_encoder.choose_reflector("B");
        _ = test_encoder.add_plugboard_wires("d-r,g-m,k-o,t-y");
        let mut test_decoder = test_encoder.clone();
        let to_encode = "this is a long message that will be encoded. woot.";
        let encoded = test_encoder.translate(to_encode);
        let decoded = test_decoder.translate(&encoded);
        assert_eq!(to_encode, decoded);
        assert_ne!(to_encode, encoded);
    }
}