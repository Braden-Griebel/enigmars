use std::io;
use enigmars::machine;
use enigmars::machine::enigma::Enigma;

fn main() {
    println!("Enigma Machine");
    // Create initial machine configuration
    let mut engima_machine = machine::enigma::Enigma::default();
    loop {
        print!(
            "Please Select an Option:
              1) Choose Rotors
              2) Configure Rotors
              3) Choose Reflector
              4) Configure Reflector
              5) Configure Plugboard
              6) Display Configuration
              7) Translate Message
              8) Quit");
        let mut choice = String::new();
        // Get Choice
        io::stdin().read_line((&mut choice)).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => { choose_rotors(&mut engima_machine); }
            2 => { configure_rotors(&mut engima_machine); }
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {
                println!("Exiting. Thank you!");
                break;
            }
            _ => { continue }
        }
    }
}

// Choose which rotors to use
fn choose_rotors(machine: &mut Enigma) {
    let mut finished = false;
    loop {
        println!("Enter your choices of rotor seperated by commas.
        choices are: I,II,III,IV,V,VI,VII,VIII");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to get choice of rotor");
        finished = true;
        for (idx, r) in choice.trim().split(",").enumerate() {
            if idx > 2 {
                println!("Too many rotors selected!");
                finished = false;
                break;
            }
            match machine.choose_rotor(r.trim(), idx as u8) {
                Ok(_) => {}
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            };
        }
        if finished { break; }
    }
}

// Choose the settings for the rotors
fn configure_rotors(machine: &mut Enigma) {
    loop {
        println!("Enter your choice of rotor settings, should be 3 characters with no commas \
        or other separation");
        let mut setting = String::new();
        io::stdin().read_line(&mut setting).expect("Failed to get choice of setting");
        match machine.set_rotors(setting.trim()) {
            Ok(_) => { break; }
            Err(err) => {
                println!("{}", err);
            }
        };
    }
}
