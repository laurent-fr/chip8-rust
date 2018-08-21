extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use vm::Vm;

pub struct Engine<'a> {
    vm: &'a mut Vm
}

const WIDTH:u32 = 64;
const HEIGHT:u32 = 32 ;
const SCALING:u32 = 12;

const BK:(u8,u8,u8) = (100,100,255);
const FK:(u8,u8,u8) = (200,200,255);


impl<'a> Engine<'a> {

    pub fn new(_vm:&'a mut Vm) -> Engine {
        Engine { vm:_vm }
    }

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo: Video", WIDTH*SCALING, HEIGHT*SCALING)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(BK.0, BK.1, BK.2));
        canvas.clear();
        canvas.present();
    
        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }
        
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            // The rest of the game loop goes here...

            // set keyboard code

            // cycle
            self.vm.cycle();

            // timers

        }
    }

    fn set_pixel(_x:u32,_y:u32) {

    }



}