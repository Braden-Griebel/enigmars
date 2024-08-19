use std::ascii::AsciiExt;

/// Represents the reflector, which turns the signal around on the right side
/// of the machine
#[derive(Clone)]
pub struct Reflector {
    configuration: [u8;26],
    offset: u8,
}

impl Reflector{

    /// Create a new reflector instance
    pub fn new(configuration: &str, setting:char)->Self{
        let configuration: [u8;26] = configuration.to_ascii_lowercase()
            .bytes().map(|x| x-97u8).collect::<Vec<u8>>().try_into()
            .unwrap_or_else(
                |v: Vec<u8>| panic!("Expected a Vec of length 26 but it was {}", v.len()));
        let offset: u8 = setting.to_ascii_lowercase() as u8 - 97u8;
        Self{
            configuration,
            offset
        }
    }

    /// Translate a character through the reflector
    pub fn translate(&self, input: char)->char{
        let input_val: u8 = input as u8 -97u8;
        let output_val: u8 = (input_val + self.offset) % 26;
        ((Self::wrap_26_sub(self.configuration[output_val as usize], self.offset))+97u8) as char
    }

    /// Change the setting of the reflector
    pub fn set(&mut self, setting: char){
        self.offset = setting.to_ascii_lowercase() as u8 - 97u8;
    }

    pub fn new_A()->Self {
        Self::new("EJMZALYXVBWFCRQUONTSPIKHGD", 'a')
    }

    pub fn new_B()->Self {
        Self::new("YRUHQSLDPXNGOKMIEBFZCWVJAT", 'a')
    }

    pub fn new_C()->Self {
        Self::new("FVPJIAOYEDRZXWGCTKUQSBNMHL", 'a')
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
mod test_reflector{
    use super::*;
    #[test]
    fn test_translate(){
        let test_reflector = Reflector::new_A();
        assert_eq!('e', test_reflector.translate('a'));
    }

    #[test]
    fn test_configuration(){
        // test that reflector configurations all must loop (so if a->e, e->a)
        let test_reflector  = Reflector::new_A();
        test_loop(&test_reflector);
        let test_reflector  = Reflector::new_B();
        test_loop(&test_reflector);
        let test_reflector  = Reflector::new_C();
        test_loop(&test_reflector);
    }

    fn test_loop(reflector: &Reflector){
        for c in 'a'..='z'{
            assert_eq!(c, reflector.translate(reflector.translate(c)))
        }
    }
}