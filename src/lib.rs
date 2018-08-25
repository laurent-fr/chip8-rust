/*  CHIP-8 Emulator in Rust
    Copyright (C) 2018 Laurent FRANCOISE @_Laurent_Fr_

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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