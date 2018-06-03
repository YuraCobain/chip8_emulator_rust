extern crate sdl2;

use media_be::sdl2::pixels::Color;
use media_be::sdl2::event::Event;
use media_be::sdl2::keyboard::Keycode;
use std::time::Duration;

pub struct Point<T> {
    x: T,
    y: T,
}

pub trait Gfx {
    fn display_sprite(&mut self, p: &[Point<u8>]) -> Option<u8>;
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
        let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
            .position_centered()
            .build()
        .unwrap();
 
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        Sdl2Be {
            sdl_ctx: sdl_context,
            video_ss: video_subsystem,
            canvas: canvas,
            ev: event_pump, 
        }
    }

    pub fn run_one_tick(&mut self) {
        let mut i = 0;
        i = (i + 1) % 255;
        self.canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        self.canvas.clear();
        for event in self.ev.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        println!("quit")
                    },
                _ => {}
            }
        }

        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

impl Gfx for Sdl2Be {
    fn display_sprite(&mut self, p: &[Point<u8>]) -> Option<u8> {
        let sdl_ps: Vec<sdl2::rect::Point> = Vec::with_capacity(p.len());
        self.canvas.draw_points(sdl_ps.as_slice());

        Some(0)
    }
}
