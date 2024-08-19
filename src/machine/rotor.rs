use std::collections::HashSet;

/// Represents the rotors of the Enigma machine. Translates characters passed into the rotor,
/// steps the rotor when required (either each letter in the case of the rightmost rotor,
/// or when the rotor to the right hits its notch).
#[derive(Clone)]
pub(crate) struct Rotor {
    /// Which letters/positions are translated by this rotor
    /// when going forward
    path_fwd: [u8; 26],
    /// Which letters/positions are translated by this rotor
    /// when going in reverse (return leg)
    path_rev: [u8;26],
    /// Current offset of the rotor from 0 (always 0..26)
    offset: u8,
    /// Which steps will cause the left neighbor rotor to step as well
    notches: HashSet<u8>,
}

impl Rotor {
    pub fn new(configuration: &str, setting: char, notches: &str) -> Self {
        let path_fwd: [u8; 26] = configuration.to_ascii_lowercase()
            .bytes().map(|x| x-97u8).collect::<Vec<u8>>().try_into()
            .unwrap_or_else(
                |v: Vec<u8>| panic!("Expected a Vec of length 26 but it was {}", v.len()));
        let offset = setting.to_ascii_lowercase() as u8 - 97u8;
        let notches: HashSet<u8> = HashSet::from_iter(notches.to_ascii_lowercase()
            .chars().map(|c| (c as u8) - 97u8));
        let mut path_rev: [u8;26] = [0;26];
        // Find the reverse paths
        for i in 0..26usize {
            let fwd = path_fwd[i];
            path_rev[fwd as usize] = i as u8;
        }
        Self {
            path_fwd,
            path_rev,
            offset,
            notches,
        }
    }

    /// Change the setting of the rotor
    pub fn set(&mut self, setting: char){
        self.offset = setting.to_ascii_lowercase() as u8 - 97u8;
    }

    /// Step the rotor, returns true if the next rotor should step as well,
    /// and false otherwise
    pub fn step(&mut self) -> bool {
        self.offset += 1;
        // The -1 here is due to the way the rotor notches are normally described,
        // the step occurs when stepping off the position rather than stepping onto it
        if self.notches.contains(&(self.offset-1)) {
            return true;
        }
        false
    }

    /// Translate a character through the rotor
    pub fn translate_forward(&self, input: char) -> char {
        let input_val: u8 = input as u8 - 97u8;
        let output_val: u8 = (input_val + self.offset) % 26;
        ((Self::wrap_26_sub(self.path_fwd[output_val as usize], self.offset)) + 97u8) as char
    }

    /// Translate a character through the rotor on the reverse leg
    pub fn translate_reverse(&self, input: char) -> char {
        let input_val: u8 = input as u8-97u8;
        let output_val: u8 = (input_val + self.offset) % 26;
        ((Rotor::wrap_26_sub(self.path_rev[output_val as usize],self.offset))+97u8) as char
    }

    // Functions to create specific Rotors
    /// Function to create a rotor with configuration I
    pub fn new_I() -> Self {
        Self::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'a', "Q")
    }

    /// Function to create a rotor with configuration II
    pub fn new_II() -> Self {
        Self::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'a', "E")
    }

    /// Function to create a rotor with configuration III
    pub fn new_III() -> Self {
        Self::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'a', "V")
    }

    /// Function to create a rotor with configuration IV
    pub fn new_IV() -> Self {
        Self::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'a', "J")
    }

    /// Function to create a rotor with configuration V
    pub fn new_V() -> Self {
        Self::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'a', "Z")
    }

    /// Function to create a rotor with configuration VI
    pub fn new_VI() -> Self {
        Self::new("JPGVOUMFYQBENHZRDKASXLICTW", 'a', "ZM")
    }

    /// Function to create a rotor with configuration VII
    pub fn new_VII() -> Self {
        Self::new("NZJHGRCXMYSWBOUFAIVLPEKQDT", 'a', "ZM")
    }

    /// Function to create a rotor with configuration VIII
    pub fn new_VIII() -> Self {
        Self::new("FKQHTLXOCBJSPDZRAMEWNIUYGV", 'a', "ZM")
    }

    fn wrap_26_sub(lhs:u8, rhs:u8)->u8{
        if rhs <= lhs {
            lhs - rhs
        } else {
            26-(rhs-lhs)
        }
    }
}

#[cfg(test)]
mod test_rotors{
    use super::*;

    #[test]
    fn test_wrap_26_sub(){
        assert_eq!(24, Rotor::wrap_26_sub(3,5));
        assert_eq!(2, Rotor::wrap_26_sub(5,3));
    }

    #[test]
    fn test_translation(){
        let test_rotor = Rotor::new_I();
        // Forward translate
        assert_eq!('a', test_rotor.translate_forward('u'));
        assert_eq!('e', test_rotor.translate_forward('a'));
        // Reverse translate
        assert_eq!('a', test_rotor.translate_reverse('e'));
        assert_eq!('m', test_rotor.translate_reverse('o'));
    }

    #[test]
    fn test_step(){
        let mut test_rotor = Rotor::new_I();
        // Check translation
        assert_eq!('e', test_rotor.translate_forward('a'));
        assert_eq!('k', test_rotor.translate_forward('b'));
        assert_eq!('n', test_rotor.translate_forward('k'));
        assert_eq!('l', test_rotor.translate_reverse('t'));
        // Step the rotor
        _ = test_rotor.step();
        // Check the translation again
        assert_eq!('j', test_rotor.translate_forward('a'));
    }

    #[test]
    fn test_notch(){
        let mut test_rotor = Rotor::new_I();
        test_rotor.offset = 16;
        assert!(test_rotor.step());
    }
}