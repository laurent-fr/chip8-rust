extern crate sdl2;
extern crate time;
extern crate rand;

const WIDTH:u32 = 64;
const HEIGHT:u32 = 32 ;
const SCALING:u32 = 12;
const SCREEN_SIZE:usize = WIDTH as usize * HEIGHT as usize / 8;
const WIDTH_BYTE:u32 = WIDTH/8;
const VBL:u64 = 1_000_000_000u64 / 60 ;
const SIMULATOR_SPEED:u64 = 1_000_000_000u64 / 100_000_000u64 ; 

pub mod engine;
pub mod vm;