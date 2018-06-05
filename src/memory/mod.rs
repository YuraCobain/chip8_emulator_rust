const ROM_START_ADDR: usize = 0;
const ROM_END_ADDR: usize = 0x1FF;
const EXE_START_ADDR: usize = 0x200;
const EXE_END_ADDR: usize = 0xFFF;
const SPRITE_SIZE: usize = 0x5;
pub const MEM_SIZE: usize = 0xFFF;
const STACK_SIZE: usize = 32;
const STACK_START_ADDR: usize = MEM_SIZE - STACK_SIZE - 1;

pub trait CpuMemory {
    fn get_font_sprite(&self, s_n: u8) -> Option<&[u8]>;
    fn get_font_sprite_addr(&self, s_n: u8) -> Option<u16>;
    fn get_sprites(&self, addr: u16, n: u8) -> Option<&[u8]>;
    
    fn get_instruction(&self, addr: u16) -> Option<u16>;
    fn set_u8(&mut self, addr: u16, val: u8) -> Option<()>;
    fn get_u8(&mut self, addr: u16) -> Option<u8>;
    
    fn push(&mut self, val: u16) -> Option<()>;
    fn pop(&mut self) -> Option<u16>;
}

#[derive(Copy)]
pub struct Memory {
    memory: [u8; MEM_SIZE],
    stack_top: usize,
    stack_len: usize,
}

impl Clone for Memory {
    fn clone(&self) -> Memory { *self }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; MEM_SIZE],
            stack_top: STACK_START_ADDR,
            stack_len: STACK_SIZE,
        }
    }

    pub fn load_exe(&mut self, exe: &[u8]) -> &mut Self {
        self.memory[EXE_START_ADDR..EXE_START_ADDR + exe.len()].clone_from_slice(exe);
        self
    }

    pub fn load_sprites(&mut self, sprites: &[u8]) -> &mut Self {
        self.memory[ROM_START_ADDR..ROM_START_ADDR + sprites.len()].clone_from_slice(sprites);
        self
    }

    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

impl CpuMemory for Memory {
    fn get_instruction(&self, addr: u16) -> Option<u16> {
        let high_byte: u16 = self.memory[addr as usize] as u16;
        let low_byte: u16 = self.memory[(addr as usize + 1)] as u16;

        Some(high_byte << 8 | low_byte)
    }

    fn get_font_sprite_addr(&self, s_num: u8) -> Option<u16> {
        Some((ROM_START_ADDR + SPRITE_SIZE * (s_num as usize)) as u16)
    }

    fn get_font_sprite(&self, s_num: u8) -> Option<&[u8]> {
        let start = ROM_START_ADDR + SPRITE_SIZE * (s_num as usize);
        let end = start + SPRITE_SIZE;
        Some(&self.memory[start..end])
    }

    fn get_sprites(&self, addr: u16, n: u8) -> Option<&[u8]> {
        let start = addr as usize;
        let end = start + n as usize;
        Some(&self.memory[start..end])
    }

    fn set_u8(&mut self, addr: u16, val: u8) -> Option<()> {
        self.memory[addr as usize] = val;
        Some(())
    }

    fn get_u8(&mut self, addr: u16) -> Option<u8> {
        Some(self.memory[addr as usize])
    }
    
    fn push(&mut self, val: u16) -> Option<()> {
        if self.stack_top == STACK_START_ADDR + STACK_SIZE  {
            return None;
        }

        self.stack_top += 2;            
        self.memory[self.stack_top] = (val >> 8) as u8 ;
        self.memory[self.stack_top + 1] = val as u8;
        
        println!("{:x} {:x}", self.memory[self.stack_top], self.memory[self.stack_top + 1]);
        Some(())
    }

    fn pop(&mut self) -> Option<u16> {
        if self.stack_top == STACK_START_ADDR {
            return None;
        }

        println!("{:x} {:x}", self.memory[self.stack_top], self.memory[self.stack_top + 1]);
        let hb = self.memory[self.stack_top] as u16;
        let lb = self.memory[self.stack_top + 1] as u16;
        self.stack_top -= 2;

        Some((hb << 8) | lb)
    }
}

pub trait VideoMemory {
    fn apply_sprites(&mut self, x: u8, y: u8, sprites: &[u8]) -> Option<u8>;
    fn get_video_buf(&mut self) -> Option<&[[u8; DISPLAY_TOTAL_WIDTH]]>;
}

const DISPLAY_VISIBLE_WIDTH: usize = 64;
const DISPLAY_VISIBLE_HEIGHT: usize = 32;

// total video memoty width in words including wrapping area
const DISPLAY_TOTAL_WIDTH: usize = DISPLAY_VISIBLE_WIDTH / 8 + 1; 
// total video memoty height in bits including wrapping area
const DISPLAY_TOTAL_HEIGHT: usize = DISPLAY_VISIBLE_HEIGHT + 4;

#[derive(Copy)]
pub struct Display {
    memory: [[u8; DISPLAY_TOTAL_WIDTH]; DISPLAY_TOTAL_HEIGHT],
}

impl Clone for Display {
    fn clone(&self) -> Display { *self }
}

impl Display {
    pub fn new() -> Self {
        Display {
            memory: [[0; DISPLAY_TOTAL_WIDTH]; DISPLAY_TOTAL_HEIGHT],
        }
    }
}

impl VideoMemory for Display {
    fn apply_sprites(&mut self, x: u8, y: u8, sprites: &[u8]) -> Option<u8> {
        let mut collision = 0u8;
       
        // calculate correct offset in bytes and bits
        let byte_offset = x / 8;
        let bit_offset = x % 8;

        let gdb = |d: &[[u8; 9]]| {
            for r in d {
                for c in r {
                    print!("{:08b} ", c);
                }
                println!("");
            }
        };

        println!("x: {}, y: {}, by_o: {}, bi_o: {}, s_len {}",                 
                 x, y, byte_offset, bit_offset, sprites.len());
        for s in 0..sprites.len() {
            let curr_r = s + y as usize;
            let mut row_bh = self.memory[curr_r][byte_offset as usize] as u16; 
            let mut row_bl = self.memory[curr_r][(byte_offset + 1) as usize] as u16; 

            let mut row = (row_bh << 8) | row_bl;
            let row_copy = row;
            let sprite_row_apply = (sprites[s] as u16) << (8 - bit_offset);

            row ^= sprite_row_apply;

            self.memory[curr_r][byte_offset as usize] = (row >> 8) as u8;
            self.memory[curr_r][(byte_offset + 1) as usize] = row as u8;
                
            if row & row_copy != row_copy {
                collision = 1;
            } 
            println!("sprite_8 {:08b}, sprite_16 {:016b}, res: {:016b}",
                     sprites[s], sprite_row_apply, row);
        }
        gdb(&self.memory[..]);

        Some(collision)
    }
    
    fn get_video_buf(&mut self) -> Option<&[[u8; DISPLAY_TOTAL_WIDTH]]> {
        Some(&self.memory[..])
    }
}



