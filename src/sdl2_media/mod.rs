extern crate sdl2;

use sdl2_media::sdl2::pixels::Color;
use sdl2_media::sdl2::event::Event;
use sdl2_media::sdl2::rect::Point;
use sdl2_media::sdl2::keyboard::Keycode;

use media_if::*;

pub struct Sdl2Be {
    _sdl_ctx: sdl2::Sdl,
    _video_ss: sdl2::VideoSubsystem,
    canvas: sdl2::render::WindowCanvas,
    ev: sdl2::EventPump,
    keypad: [u8; 16],
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
        canvas.set_scale(10.0, 10.0).unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        Sdl2Be {
            _sdl_ctx: sdl_context,
            _video_ss: video_subsystem,
            canvas: canvas,
            ev: event_pump, 
            keypad: [0; 16],
        }
    }

    fn evnt2code(kc: Keycode) -> Option<u8> {
        let key = match kc {
            Keycode::Num0 => Some(0),
            Keycode::Num1 => Some(1),
            Keycode::Num2 => Some(2),
            Keycode::Num3 => Some(3),
            Keycode::Num4 => Some(4),
            Keycode::Num5 => Some(5),
            Keycode::Num6 => Some(6),
            Keycode::Num7 => Some(7),
            Keycode::Num8 => Some(8),
            Keycode::Num9 => Some(9),
            Keycode::A => Some(0xa),
            Keycode::B => Some(0xb),
            Keycode::C => Some(0xc),
            Keycode::D => Some(0xd),
            Keycode::E => Some(0xe),
            Keycode::F => Some(0xf),
            _ => None,
        };

        key
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
                    }
                }
            }
        }

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.draw_points(sdl_ps.as_slice()).unwrap();

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
    
    fn process_events(&mut self) -> bool {
        for event in self.ev.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                => { 
                    return false;
                },

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    match Sdl2Be::evnt2code(keycode) {
                        Some(kcode) => self.keypad[kcode as usize] = 1,
                        None => {},
                    }
                },
                Event::KeyUp {keycode: Some(keycode), ..} => {
                    match Sdl2Be::evnt2code(keycode) {
                        Some(kcode) => self.keypad[kcode as usize] = 0,
                        None => {},
                    }
                },
                _ => {}
            }
        }

        return true;
    }
    
    fn is_key_pressed(&mut self, key: u8) -> bool {
        self.keypad[key as usize] == 1
    }
    
    fn get_pressed_key(&self) -> Option<&u8> {
        self.keypad.iter().find(|&&s| s == 1)
    }
}
