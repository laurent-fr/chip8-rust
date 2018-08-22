extern crate sdl2;
extern crate time;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use std::time::Duration;



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
        let audio_subsystem = sdl_context.audio().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("CHIP-8 Emulator", ::WIDTH*::SCALING, ::HEIGHT*::SCALING)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(BK.0, BK.1, BK.2));
        canvas.clear();
        canvas.present();
    
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut now = time::precise_time_ns();

        self.draw_screen(&mut canvas);

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {

                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },

                    Event::KeyDown {keycode: Some(keycode), ..} => {
                        match keycode {
                            Keycode::Num1 => self.vm.key = 0 ,
                            Keycode::Num2 => self.vm.key = 1 ,
                            Keycode::Num3 => self.vm.key = 2 ,
                            Keycode::Num4 => self.vm.key = 3 ,
                            Keycode::A => self.vm.key = 4 ,
                            Keycode::Z => self.vm.key = 5 ,
                            Keycode::E => self.vm.key = 6 ,
                            Keycode::R => self.vm.key = 7 ,
                            Keycode::A => self.vm.key = 8 ,
                            Keycode::S => self.vm.key = 9 ,
                            Keycode::D => self.vm.key = 10 ,
                            Keycode::F => self.vm.key = 11 ,
                            Keycode::W => self.vm.key = 12 ,
                            Keycode::X => self.vm.key = 13 ,
                            Keycode::C => self.vm.key = 14 ,
                            Keycode::V => self.vm.key = 15,
                            _ => self.vm.key = -1
                        }
                    },

                    Event::KeyUp {keycode: Some(_keycode), ..} => {
                        self.vm.key = -1
                    },

                    _ => {}
                }
            }
        
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // game loop ...

            

            // cycle
            if self.vm.cycle() == true {
                self.draw_screen(&mut canvas);
            }

            // timers
            if now + ::VBL < time::precise_time_ns() {
                now = time::precise_time_ns();
                self.vm.update_timers();
            }
            // sound

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
            for col in 0 .. ::WIDTH/8 {
                let addr = (line*(::WIDTH/8) + col) as usize;
            
                self.draw_byte(canvas,col as i32,line as i32,self.vm.screen[addr]);
            }
        }
        canvas.present();
    }


}