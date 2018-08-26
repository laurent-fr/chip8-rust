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

// see https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/audio-squarewave.rs

use sdl2::audio::AudioCallback;

pub struct SquareWave {
    pub phase_inc: f32,
    pub phase: f32,
    pub volume: f32
}


impl AudioCallback for SquareWave {

    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 { self.volume } else { -self.volume };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}