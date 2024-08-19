use enigmars::machine;
use enigmars::machine::enigma::Enigma;
use std::io;
use std::io::Write;

fn main() {
    println!("Enigma Machine");
    // Create initial machine configuration
    let mut enigma_machine = machine::enigma::Enigma::default();
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
              8) Quit\n");
        let mut choice = String::new();
        // Get Choice
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => { choose_rotors(&mut enigma_machine); }
            2 => { configure_rotors(&mut enigma_machine); }
            3 => { choose_reflector(&mut enigma_machine); }
            4 => { configure_reflector(&mut enigma_machine);}
            5 => { configure_plugboard(&mut enigma_machine);}
            6 => {
                println!("{}", enigma_machine);
            }
            7 => {
                translate_message(&mut enigma_machine.clone());
            }
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
    let mut finished = true;
    loop {
        println!("Enter your choices of rotor seperated by commas.
        choices are: I,II,III,IV,V,VI,VII,VIII");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to get choice of rotor");
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

fn choose_reflector(machine: &mut Enigma){
    loop {
        println!("Please enter choice for reflector.
        Choices are a,b,c (case insensitive)");
        let mut reflector = String::new();
        io::stdin().read_line(&mut reflector).expect("Failed to get choice for reflector");
        if reflector.trim().len() == 0 {
            break;
        }
        match machine.choose_reflector(reflector.trim()) {
            Ok(_) => {break;}
            Err(_) => {
                println!("Error selecting reflector, please try again (or enter a blank string to return to main menu)");
                continue;
            }
        }
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
                continue;
            }
        };
    }
}

// choose the setting for the reflector
fn configure_reflector(machine: &mut Enigma){
    loop {
        println!("Enter your choice of reflector setting, should be a single character.");
        let mut setting = String::new();
        io::stdin().read_line(&mut setting).expect("Failed to get choice of reflector setting");
        match machine.set_reflector(setting.trim()){
            Ok(_)=>{break;}
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
}

// Plugboard configuration
fn configure_plugboard(machine: &mut Enigma){
    loop{
        print!("Please Select an Option
          1) Add Wires to the Plugboard
          2) Remove Wires from the Plugboard
          3) Return to the Main Menu\n");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to get choice in plugboard configuration");
        let choice = match choice.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{continue},
        };

        match choice {
            1=>{add_wires(machine);}
            2=>{remove_wires(machine);}
            3=>{break;}
            _=>{continue;}
        };

    }
}

// Add wires to the plugboard
fn add_wires(machine: &mut Enigma){
    loop{
        println!("Enter the wires you would like to add as comma seperated list of \
        <start>-<end>. For example 'a-e,b-g,q-l' would add wires connected a to e, bto g, and q to l.");
        let mut wires = String::new();
        io::stdin().read_line(&mut wires).expect("Failed to read wires to add to plugboard");
        match machine.add_plugboard_wires(&wires){
            Ok(_) => {break;}
            Err(_) => {
                println!("Failed to add wires to board, try again (or enter blank string to exit)");
                continue;
            }
        }
    }
}

// remove wires from plugboard
fn remove_wires(machine: &mut Enigma){
    loop {
        println!("Enter the wires you would like to remove as a comma seperated list of \
        characters. The characters should represent 1 end of a wire to remove. For example, \
        if the plugboard has wires connected a to e, b to q, and l to m entering 'e,q' will \
        remove the wires connecting a to e, and b to q.");
        let mut to_remove = String::new();
        io::stdin().read_line(&mut to_remove).expect("Failed to read wires to remove from plugboard");
        match machine.remove_plugboard_wires(&to_remove){
            Ok(_) => {break;}
            Err(_) => {
                println!("Error removing wires, try again!");
                continue;
            }
        };
    }
}

// translate message
fn translate_message(machine: &mut Enigma){
    println!("Enter the message you would like to translate:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read message to translate");
    println!("{}", machine.translate(&message));
}
