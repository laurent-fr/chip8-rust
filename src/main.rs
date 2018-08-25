extern crate chip8;

use chip8::engine::Engine;
use chip8::vm::Vm;

use std::fs::File;
use std::io::Read;
use std::env;

fn main() {

    println!("CHIP-8 Emulator");

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
    }

    let mut vm = Vm::new();
    load_rom(&mut vm,&args[1]);
    let mut engine = Engine::new(&mut vm);

    engine.run();



}

fn load_rom(vm:&mut Vm,filename: &String) {
    let mut file=File::open(filename).unwrap();
    let mut buf=[0u8;4096];
    file.read(&mut buf).unwrap();

    for i in 0x200..0xfff as usize {
        vm.mem[i]=buf[i-0x200];
    }

}

fn usage() {
    println!("Usage: chip8 [rom file]");
    ::std::process::exit(0);
}