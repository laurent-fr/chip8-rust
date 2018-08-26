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

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use sdl2::audio::AudioSpecDesired;

use std::time::Duration;

use audio::SquareWave;
use vm::Vm;

pub struct Engine<'a> {
    vm: &'a mut Vm
   
}

const BK:(u8,u8,u8) = (100,100,255);
const FK:(u8,u8,u8) = (210,210,255);


impl<'a> Engine<'a> {

    pub fn new(_vm:&'a mut Vm) -> Engine {
        Engine { vm:_vm }
    }

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();

        // initialize SDL2 video
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("CHIP-8 Emulator", ::WIDTH*::SCALING, ::HEIGHT*::SCALING)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        // initialize SDL2 audio
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();

        // clear screen
        canvas.set_draw_color(Color::RGB(BK.0, BK.1, BK.2));
        canvas.clear();
        canvas.present();
    
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut now = time::precise_time_ns();

        // main loop
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {

                    // ORIGINAL     PC KEYBOARD
                    // 1 2 3 C      1 2 3 4
                    // 4 5 6 D      A Z E R
                    // 7 8 9 E      Q S D F
                    // A 0 B F      W X C V

                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        match keycode {
                            Keycode::Num1 => self.vm.key = 0x01 ,
                            Keycode::Num2 => self.vm.key = 0x02 ,
                            Keycode::Num3 => self.vm.key = 0x03 ,
                            Keycode::Num4 => self.vm.key = 0x0c ,
                            Keycode::A => self.vm.key = 0x04 ,
                            Keycode::Z => self.vm.key = 0x05 ,
                            Keycode::E => self.vm.key = 0x06 ,
                            Keycode::R => self.vm.key = 0x0d ,
                            Keycode::Q => self.vm.key = 0x07 ,
                            Keycode::S => self.vm.key = 0x08 ,
                            Keycode::D => self.vm.key = 0x09 ,
                            Keycode::F => self.vm.key = 0x0e ,
                            Keycode::W => self.vm.key = 0x0a ,
                            Keycode::X => self.vm.key = 0x00 ,
                            Keycode::C => self.vm.key = 0x0b ,
                            Keycode::V => self.vm.key = 0x0f ,
                            Keycode::Escape => break 'running ,
                            _ => self.vm.key = -1
                        }
                    },

                    Event::KeyUp {keycode: Some(_keycode), ..} => {
                        self.vm.key = -1
                    },

                     Event::Quit {..}  => {
                        break 'running
                    }

                    _ => {}
                }
            }
        
            ::std::thread::sleep(Duration::new(0, ::SIMULATOR_SPEED as u32));
           

            //self.vm.debug();            

            // cycle
            if self.vm.cycle() == true {
                self.draw_screen(&mut canvas);
            }

            // timers
            if now + ::VBL < time::precise_time_ns() {
                now = time::precise_time_ns();
                self.vm.update_timers();
                 canvas.present();
            }

            // sound
            if self.vm.reg_st>0 {
                audio_device.resume();
            } else {
                audio_device.pause();
            }

        }
    }

    fn draw_byte(&self, canvas:&mut Canvas<Window>,x:i32,y:i32,data:u8) {
            let mut d = data;
            for i in 0..8 {
                if d&0x80 == 0x80 {
                    canvas.set_draw_color(Color::RGB(FK.0, FK.1, FK.2));
                } else {
                    canvas.set_draw_color(Color::RGB(BK.0, BK.1, BK.2));
                }
                canvas.fill_rect(Rect::new( (x*8+i)*::SCALING as i32, y *::SCALING as i32, ::SCALING, ::SCALING)).unwrap();
                d <<=1 ;
            }
    }

    fn draw_screen(&self,canvas:&mut Canvas<Window>) {

        for line in 0 .. ::HEIGHT  {
            for col in 0 .. ::WIDTH_BYTE {
                let addr = (line*::WIDTH_BYTE + col) as usize;
            
                self.draw_byte(canvas,col as i32,line as i32,self.vm.screen[addr]);
            }
        }
       
    }


}


