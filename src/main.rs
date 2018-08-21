extern crate chip8;

use chip8::engine::Engine;
use chip8::vm::Vm;

fn main() {

    println!("CHIP-8 Emulator");

    let mut vm = Vm::new();
    let mut engine = Engine::new(&mut vm);

    engine.run();



}
