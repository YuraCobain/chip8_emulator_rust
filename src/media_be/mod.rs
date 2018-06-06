extern crate sdl2;

use media_be::sdl2::pixels::Color;
use media_be::sdl2::event::Event;
use media_be::sdl2::rect::Point;
use media_be::sdl2::keyboard::Keycode;

use std::time::Duration;

pub trait MediaIf {
    fn draw_display(&mut self, buf: &[[u8; 10]]) -> Option<u8>;
    fn clear_display(&mut self) -> Option<u8>;
    fn present_display(&mut self) -> Option<u8>;

    fn wait_key_press(&mut self) -> Option<u8>;
    fn is_key_pressed(&mut self, key: u8) -> bool;
}

pub struct Sdl2Be {
    sdl_ctx: sdl2::Sdl,
    video_ss: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
    ev: sdl2::EventPump,
}

impl Sdl2Be {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("rust-sdl2 demo", 640, 320)
            .position_centered()
            .build()
        .unwrap();
 
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_scale(10.0, 10.0);
        let mut event_pump = sdl_context.event_pump().unwrap();

        Sdl2Be {
            sdl_ctx: sdl_context,
            video_ss: video_subsystem,
            canvas: canvas,
            ev: event_pump, 
        }
    }

    fn evnt2code(e: Event) -> Option<u8> {
        let key = match e {
            Event::KeyDown { keycode: Some(Keycode::Num0), .. } => {
                println!("key 0");
                Some(0)
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                println!("key 1");
                Some(1)
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                println!("key 2");
                Some(2)
            },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                println!("key 3");
                Some(3)
            },
            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                println!("key 4");
                Some(4)
            },
            Event::KeyDown { keycode: Some(Keycode::Num5), .. } => {
                println!("key 5");
                Some(5)
            },
            Event::KeyDown { keycode: Some(Keycode::Num6), .. } => {
                println!("key 6");
                Some(6)
            },
            Event::KeyDown { keycode: Some(Keycode::Num7), .. } => {
                println!("key 7");
                Some(7)
            },
            Event::KeyDown { keycode: Some(Keycode::Num8), .. } => {
                println!("key 8");
                Some(8)
            },
            Event::KeyDown { keycode: Some(Keycode::Num9), .. } => {
                println!("key 9");
                Some(9)
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                println!("key a");
                Some(0xa)
            },
            Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                println!("key b");
                Some(0xb)
            },
            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                println!("key c");
                Some(0xc)
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                println!("key d");
                Some(0xd)
            },
            Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                println!("key e");
                Some(0xE)
            },
            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                println!("key f");
                Some(0xf)
            },
            _ => {
                None
            }
        };

        key
    }

    pub fn run_one_tick(&mut self) {
        for event in self.ev.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        println!("quit")
                    },
                _ => {}
            }
        }
    }
}

fn test_nth_bit(x: u8, nth: u8) -> bool {
    (x & (1 << (7 - nth))) != 0
}

impl MediaIf for Sdl2Be {
    fn draw_display(&mut self, buf: &[[u8; 10]]) -> Option<u8> {
        let mut sdl_ps: Vec<Point> = Vec::with_capacity(32*64);

        for r in 0..buf.len() {
            let row = buf[r];
            for c in 0..row.len() {
                for b in 0..8 {
                    if test_nth_bit(row[c], b) {
                        let p = Point::new(((c*8 + b as usize) as u8) as i32, r as i32);
                        sdl_ps.push(p);
                        println!("point {:?}", p);
                        println!("r {} c {} b {}", r, c, b);
                    }
                }
            }
        }

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.draw_points(sdl_ps.as_slice());

        Some(0)
    }

    fn clear_display(&mut self) -> Option<u8> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        Some(0)
    }

    fn present_display(&mut self) -> Option<u8> {
        self.canvas.present();
        Some(0)
    }
    
    fn wait_key_press(&mut self) -> Option<u8> {
        loop {
            match Sdl2Be::evnt2code(self.ev.wait_event()) { 
                Some(e) => return Some(e),
                None => continue,
            }
        }
    }
    
    fn is_key_pressed(&mut self, key: u8) -> bool {
        for event in self.ev.poll_iter() {
            let kcode = Sdl2Be::evnt2code(event);
            if kcode.is_none() {
                continue;
            }

            if kcode.unwrap() == key {
                return true;
            }
        }

        return false;
    }
}
