use std::collections::HashSet;

use crate::machine::rotor;
use crate::machine::plugboard;
use crate::machine::reflector;

/// Struct representing the Enigma Machine
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
    pub fn set_rotor(&mut self, rotor: &str, position: u8) -> Result<(), Err(EnigmaError)> {
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
    pub fn set_reflector(&mut self, reflector: &str) -> Result<(), Err(EnigmaError)> {
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
    pub fn add_plugboard_wire(&mut self, wire:&str)->Result<(), Err(EnigmaError)>{
        let letters: Vec<char> = wire.split("-").collect();
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
    pub fn add_plugboard_wires(&mut self, wires:&str)->Result<(), Err(EnigmaError)>{
        for wire in wires.split(","){
            self.add_plugboard_wire(wire)?
        }
        Ok(())
    }

    pub fn remove_plugboard_wire(&mut self, wire: char){
        
    }

    fn translate_char(input:char)->char{

    }
}

enum EnigmaError {
    InvalidRotor(String),
    InvalidReflector(String),
    InvalidPlugboardWire(String),
    PlugboardWireOverlap(String),
}