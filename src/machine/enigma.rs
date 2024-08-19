use std::collections::HashSet;

use crate::machine::rotor;
use crate::machine::plugboard;
use crate::machine::reflector;

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

impl Enigma {
    /// Create a new enigma machine with the provided rotors, plugboard, and reflector
    pub fn new(r1: rotor::Rotor, r2: rotor::Rotor, r3: rotor::Rotor,
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
        let rotors: [rotor::Rotor; 3] = [rotor::Rotor::new_I(),
            rotor::Rotor::new_II(), rotor::Rotor::new_III()];
        let plugboard = plugboard::Plugboard::new();
        let reflector: reflector::Reflector = reflector::Reflector::new_A();
        Self {
            rotors,
            plugboard,
            reflector,
        }
    }

    /// Set a rotor in a given position
    pub fn set_rotor(&mut self, rotor: &str, position: u8) -> Result<(), EnigmaError> {
        match rotor {
            "I" => {
                self.rotors[position as usize] = rotor::Rotor::new_I();
                Ok(())
            }
            "II" => {
                self.rotors[position as usize] = rotor::Rotor::new_II();
                Ok(())
            }
            "III" => {
                self.rotors[position as usize] = rotor::Rotor::new_III();
                Ok(())
            }
            "IV" => {
                self.rotors[position as usize] = rotor::Rotor::new_IV();
                Ok(())
            }
            "V" => {
                self.rotors[position as usize] = rotor::Rotor::new_V();
                Ok(())
            }
            "VI" => {
                self.rotors[position as usize] = rotor::Rotor::new_VI();
                Ok(())
            }
            "VII" => {
                self.rotors[position as usize] = rotor::Rotor::new_VII();
                Ok(())
            }
            "VIII" => {
                self.rotors[position as usize] = rotor::Rotor::new_VIII();
                Ok(())
            }
            r => {
                Err(EnigmaError::InvalidRotor(r.to_string()))
            }
        }
    }

    /// Set a particular reflector
    pub fn set_reflector(&mut self, reflector: &str) -> Result<(), EnigmaError> {
        match reflector {
            "a" | "A" => {
                self.reflector = reflector::Reflector::new_A();
                Ok(())
            }
            "b" | "B" => {
                self.reflector = reflector::Reflector::new_B();
                Ok(())
            }
            "c" | "C" => {
                self.reflector = reflector::Reflector::new_C();
                Ok(())
            }
            r => {
                Err(EnigmaError::InvalidReflector(r.to_string()))
            }
        }
    }

    /// Add a single wire to the plugboard, should be a string of the form "<start>-<end>",
    /// for example "a-e" connects a and e on the plugboard
    pub fn add_plugboard_wire(&mut self, wire:&str)->Result<(), EnigmaError>{
        let mut letters: Vec<char> = Vec::new();
        for s in wire.split("-"){
            if s.len() == 1{
                letters.push(s.chars().next()
                    .expect("Error in converting length 1 str to char"))
            } else {
                return Err(EnigmaError::InvalidPlugboardWire(wire.to_string()))
            }
        }
        if letters.len()!=2{
            return Err(EnigmaError::InvalidPlugboardWire(wire.to_string()));
        }
        match self.plugboard.add_wire(letters[0], letters[1]) {
            Ok(())=>{Ok(())},
            Err(err) => {
                match err{
                    plugboard::PlugboardError::OverlappingWires => {
                        Err(EnigmaError::PlugboardWireOverlap(
                            format!("Couldn't add wire due to overlap {}",wire).to_string()))
                    }
                    _ => {panic!("Unknown error when inserting plugboard wire!")}
                }
            }
        }
    }

    /// Add a set of wires to the plugboard, should be a string of the form
    /// "wire,wire,wire,..." where each wire is a string of the form
    /// "<start>-<end>. For example, the string "a-e,b-d,x-z" will
    /// add connections between a and e; b and d; and x and z.
    pub fn add_plugboard_wires(&mut self, wires:&str)->Result<(), EnigmaError>{
        for wire in wires.split(","){
            self.add_plugboard_wire(wire)?
        }
        Ok(())
    }

    /// Remove a wire from the plugboard by specifying one of its ends
    pub fn remove_plugboard_wire(&mut self, wire: char)->Result<(), EnigmaError>{
        let end  = self.plugboard.translate_char(wire);
        match self.plugboard.remove_wire(wire, end){
            Ok(())=>Ok(()),
            Err(_) => {
                Err(EnigmaError::PlugboardWireRemoveFailure(
                    format!("Couldn't remove {}", wire).to_string()
                ))
            }
        }
    }

    /// Translate a string through the Enigma machine
    pub fn translate(&mut self, input: &str)->String{
        let mut translated_str: String = String::new();
        for c in input.chars(){
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
                other => {translated_str.push(other);}
            }

        }
        translated_str
    }

    /// Pass a single character through the engima machine
    fn translate_char(&self, input:char)->char{
        let mut transfer_char: char = input;
        // Pass through plugboard
        transfer_char = self.plugboard.translate_char(transfer_char);
        // Forward pass through rotors
        for r in self.rotors.iter(){
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
    fn step(&mut self){
        let mut to_step: bool = true;
        for r in self.rotors.iter_mut().rev() {
            if to_step {
                to_step = r.step();
            } else {
                to_step = false;
            }
        }
    }
}

enum EnigmaError {
    InvalidRotor(String),
    InvalidReflector(String),
    InvalidPlugboardWire(String),
    PlugboardWireOverlap(String),
    PlugboardWireRemoveFailure(String),
}

#[cfg(test)]
mod test_enigma{
    use std::arch::x86_64::_mm256_blend_epi16;
    use crate::machine::enigma::Enigma;

    #[test]
    fn test_translation(){
        let mut test_encoder = Enigma::default();
        _= test_encoder.set_reflector("B");
        let encoded = test_encoder.translate("aaaa");
        let mut test_decoder = Enigma::default();
        _=test_decoder.set_reflector("B");
        let decoded = test_decoder.translate(&encoded);
        assert_ne!(encoded, decoded);
        assert_ne!("aaaa", encoded);
        assert_eq!("aaaa", decoded);
        assert_eq!("nqxh", encoded);
    }

    #[test]
    fn test_plugboard(){
        let mut test_encoder = Enigma::default();
        _=test_encoder.set_reflector("C");
        _=test_encoder.add_plugboard_wires("a-e,b-c,x-z,m-n");

        let mut test_decoder = test_encoder.clone();

        let encoded = test_encoder.translate("this is a coded message");
        let decoded = test_decoder.translate(&encoded);

        assert_eq!("this is a coded message", decoded);
        assert_ne!(encoded, "this is a coded message");
    }
}