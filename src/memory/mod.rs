const ROM_START_ADDR: usize = 0;
const ROM_END_ADDR: usize = 0x1FF;
const EXE_START_ADDR: usize = 0x200;
const EXE_END_ADDR: usize = 0xFFF;
const SPRITE_SIZE: usize = 0x5;
pub const MEM_SIZE: usize = 0xFFF;

#[derive(Copy)]
pub struct Memory {
    memory: [u8; MEM_SIZE],
}

pub trait MemoryBus {
    fn get_instruction(&self, addr: usize) -> Option<u16>;
    fn get_sprite_addr(&self, snum: u8) -> Option<usize>;
}

impl Clone for Memory {
    fn clone(&self) -> Memory { *self }
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: [0; MEM_SIZE],
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

impl MemoryBus for Memory {
    fn get_instruction(&self, addr: usize) -> Option<u16> {
        let high_byte: u16 = self.memory[addr] as u16;
        let low_byte: u16 = self.memory[(addr + 1)] as u16;

        Some(high_byte << 8 | low_byte)
    }

    fn get_sprite_addr(&self, snum: u8) -> Option<usize> {
        Some(ROM_START_ADDR + (snum as usize)*SPRITE_SIZE)
    }
}

