extern crate sdl2;

const WIDTH:u32 = 64;
const HEIGHT:u32 = 32 ;
const SCALING:u32 = 12;
const SCREEN_SIZE:usize = WIDTH as usize * HEIGHT as usize / 8;

pub mod engine;
pub mod vm;