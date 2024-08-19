# Enigmars
This is an implementation of a basic version of the Enigma machine,
an electromechanical cryptographic device used by the Germans during WWII.
It was ultimately cracked by the British at Bletchley park, whose
code breaking efforts significantly shortened the war and saved many lives.
See [https://en.wikipedia.org/wiki/Enigma_machine](https://en.wikipedia.org/wiki/Enigma_machine),
[https://www.iwm.org.uk/history/how-alan-turing-cracked-the-enigma-code](https://www.iwm.org.uk/history/how-alan-turing-cracked-the-enigma-code),
and [https://bletchleypark.org.uk/our-story/enigma/](https://bletchleypark.org.uk/our-story/enigma/) for more information.

# Background
The Enigma machine used a plug board, rotors, and a reflecting wheel
to encode a string of characters. In order to send encoded messages,
the settings of the machine would need to be agreed upon by both
the sender and the receiver in advance. Normally this would happen
with shared code books which would describe the settings for a
particular date. These settings included which of the 8 rotors to use,
which reflector to use, the plug board setting, and the initial
rotation of each of the rotors, and reflector.

## Plug board
The plug board used wires to connect two characters together,
which would then be swapped during the encoding. So if a
wire connected a and e, then each time 'a' was encoded it would be first
swapped to an 'e', and 'e' would similarly be swapped to an 'a'.
Note that due to the reflector, this swapping would occur twice
during each character encoding.

## Rotors
Each rotor translated an incoming character into another,
and could also "step", changing which positions would be translated.
The rotors also has a setting, which describes their starting position.
The early Enigma machine used 3 rotors (which is represented here,
later naval versions used 4 rotors and a thin reflector), arranged in
sequence left to right. Each time a key was pressed current would
flow through each of the rotors, then the reflector wheel, and back
through the rotors in the other direction. Following the key press
the rightmost rotor would step, then if it stepped past a notch,
its left neighbor would similarly step (then if that neighbor
also stepped past a notch, the next rotor would step as well).
This turned the simple substitution cipher of each rotor into a
cipher that changed after each keypress.

## Reflector
In order to easily encode and decode messages, it is useful if you
can simply feed encoded text back through a machine (with the same
initial settings) and retrieve the decoded text. To allow for this,
the Enigma machine used a reflector which passed the current back through
the rotors and plug board before it encoded a letter. This looping
back through the machine meant that (as long as both machines
were set up in the same way) encoded text could simply be fed
back into the machine to decode it.

# Usage
In order to use this application, you will need to build it from source (
no binary releases are currently available). To do this, you need to
first install the rust toolchain. See [https://www.rust-lang.org/](https://www.rust-lang.org/).

Then clone this repository, and run
```shell
cargo build --release
```
from the root directory.

The binary enigmars can then be found in the enigmars/target/release directory
and can be run with
```shell
./enigmars
```
which will start the application.
