mod memory;
mod sprites;
mod utils;
mod media_if;
mod sdl2_media;
mod cpu_ops;
mod cpu;

use cpu_ops::*;
use cpu::*;
use memory::*;
use sprites::*;
use media_if::*;
use sdl2_media::*;

use std::io::prelude::*;

use std::env;
use std::fs::File;

fn load_game(path: String) -> Vec<u8> {
    let mut f = File::open(path).unwrap();
    let metadata = f.metadata().unwrap();
    let fsize = metadata.len() as usize;
    let mut exe = Vec::with_capacity(fsize);

    f.read_to_end(&mut exe).unwrap();

    let u16_bus = unsafe {
        std::mem::transmute::<&[u8], &[u16]>(exe.as_slice())
    };
    let mut del = 0u16;
    for i in 0..fsize/2 {
        if del % 16 == 0 {
            println!("");
        }

        print!("{:04X}  ", u16_bus[i]);
        del += 1;    
    }

    exe
}

use std::time::Duration;
fn execute_vm<P: PipeLine>(pl: &mut P, _c: u32) -> Option<()> {

    while pl.process_events() {
        let instruction = pl.fetch().unwrap();
        let (id, arg) = pl.decode(instruction).unwrap();
        let _ = pl.execute(id, arg).unwrap();
        pl.update_timers();
        ::std::thread::sleep(Duration::new(0, 3000000));
    }

    Some(())
}

fn main() {
    let path = env::args().nth(1).unwrap();
    let c = env::args().nth(2).unwrap().parse::<u32>().unwrap();
    let exe = load_game(path);

    let mut mem = Memory::new()
        .load_sprites(SPRITES)
        .load_exe(exe.as_slice())
        .build();

    let mut display = Display::new();
    let mut media_if = Sdl2Be::new();

    let mut emulator = CPU::new(&mut mem as &mut CpuMemory,
                                &mut display as &mut VideoMemory,
                                &mut media_if as &mut MediaIf);
    execute_vm(&mut emulator, c);
}
