extern crate sdl2;

use media_be::sdl2::pixels::Color;
use media_be::sdl2::event::Event;
use media_be::sdl2::keyboard::Keycode;
use media_be::sdl2::rect::Point;
use std::time::Duration;

pub trait MediaIf {
    fn draw_display(&mut self, mem: &[u64]) -> Option<u8>;
    fn clear_display(&mut self) -> Option<u8>;
    fn present_display(&mut self) -> Option<u8>;
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

fn test_nth_bit(x: u64, nth: u8) -> bool {
    (x & (1 << nth)) != 0
}

impl MediaIf for Sdl2Be {
    fn draw_display(&mut self, mem: &[u64]) -> Option<u8> {
        let mut sdl_ps: Vec<Point> = Vec::with_capacity(32*64);

        for i in 0..mem.len() {
            let row = mem[i];
            println!("row {:064b}", row);
            for j in 0..64 {
                if test_nth_bit(row, j) {
                    let p = Point::new((63 - j) as i32, i as i32);
                    sdl_ps.push(p);
                    println!("point {:?}", p);
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
}
