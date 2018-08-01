# Chip8-Rust
## Overview
I've been interested in emulation for a while now; I use emulators such as Dolphin and Project64 on a regular basis. Therefore, I began this project out of pure curiosity as to how emulation worked. In general, creating this kind of software for most systems such as the Gamecube or even the NES can be a daunting task. The Chip-8 is arguably one of the easiest architectures to emulate, so I figured it would be a good start. The documentation for the Chip-8 was very clear and simple, so most of the opcodes were easy to implement; however, drawing is still proving to be the hardest opcode to implement. I had a lot of help from the documentation, an article, and code from another repository to aid me on my quest. I hope you find this project to be helpful and interesting, and I hope it inspires you to get into emulation yourself!
## Features
* Emulates any application that can be run natively on the Chip-8 interpreter
* Allows the user to specify an application to emulate using the command line
* Graphics and sound are rendered using SDL
## Prerequisites
This program relies on both the Rust programming language and SDL to be installed in order to work.
* [Learn how to install Rust here](https://www.rust-lang.org/en-US/install.html)
* [Learn how to install SDL here](https://github.com/Rust-SDL2/rust-sdl2)
## Usage
1. First, clone the repository to your local machine by typing `git clone https://github.com/grantlindberg4/chip8-rust.git`.
2. Locate the folder where you cloned the repository and type `cd chip8-rust`.
3. Type `cargo run rom`, where rom is the application you wish to run.
4. Enjoy the results! Hit escape or close the window to exit the application at any time.

Chip-8 Keypad-to-Keyboard Conversion Chart

Chip-8 Keypad:
```
+---+---+---+---+
| 1 | 2 | 3 | C |
+---+---+---+---+
| 4 | 5 | 6 | D |
+---+---+---+---+
| 7 | 8 | 9 | E |
+---+---+---+---+
| A | 0 | B | F |
+---+---+---+---+
```

Keyboard:
```
+---+---+---+---+
| 1 | 2 | 3 | 4 |
+---+---+---+---+
| Q | W | E | R |
+---+---+---+---+
| A | S | D | F |
+---+---+---+---+
| Z | X | C | V |
+---+---+---+---+
```

## Notes
The draw method of this emulator is still incomplete, and many applications do not render properly as a result; pong, however, should run fine. The Chip-8 is known to have a major problem with flickering, so unfortunately there is no good way of mitigating this effect. In addition, I would like to work on fine-tuning the clock speed; while this has not necessarily proved to be a problem with the Chip-8 in particular, other systems (NES, Gamecube, etc.) may simply not run well because of it. I suggest you look at the credits for resources if you wish to go about creating your own Chip-8 emulator.
## Credits
* A huge thank-you to Laurence Muller from multigesture.net, whose article inspired me to make this emulator: http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
* A huge thank-you to the documentation for the Chip-8, as it was simple and well-explained: https://en.wikipedia.org/wiki/CHIP-8
* A huge thank-you to ev-wilt, whose Rust code was a solid reference to help me fix some nuances with my opcodes and use SDL properly: https://github.com/ev-wilt/rusty_chip_8
