# CHIP-8 Emulator in Rust

CHIP-8 is an early virtual machine (1978) running games on the COSMAC ELF, a late 70's micro-computer powered by the CDP 1802 CPU. (https://en.wikipedia.org/wiki/COSMAC_ELF)

Graphics are 1 color 64x32 pixels, the input device is a 4x4 keyboard :

    ORIGINAL     PC KEYBOARD
    1 2 3 C      1 2 3 4
    4 5 6 D      A Z E R
    7 8 9 E      Q S D F
    A 0 B F      W X C V

Please note that the mapping is for an AZERTY keyboard, if your keyboard is different you can easily change this in the engine.rs file

## Compile

* you will need RUST obviously : https://www.rust-lang.org/fr-FR/install.html
* and SDL2 : https://github.com/Rust-SDL2/rust-sdl2

then, at the same level than cargo.toml, run the command : 

    cargo build

## Running

You will  need a CHIP-8 ROM, a few are included here, copyrights belongs to their respective authors ...

To test, run the command : 

    cargo run *rom_file*

Note : which key does what is specific to each game ...

## Reference

* CHIP-8 Wikipedia : https://en.wikipedia.org/wiki/CHIP-8
* CHIP-8 Specification : http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
