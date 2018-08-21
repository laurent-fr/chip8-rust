extern crate chip8;

use chip8::graphics::Graphics;

fn main() {

    println!("CHIP-8 Emulator");

    let graph = Graphics::new();

    graph.run();

}
