extern crate rand;

mod stack;
mod memory;
mod sprites;
mod utils;

use stack::*;
use memory::*;
use sprites::*;
use utils::*;

use std::io::prelude::*;

use std::fs::File;
use std::collections::HashMap;

type ArgOctets = (u8, u8, u8, u8);
type OpCodeExe = fn(ctx: &mut CPU, arg: ArgOctets) -> Option<()>;
type Id = u16;

struct OpCodeHandler {
    name: &'static str,
    executor: OpCodeExe,
}

struct ISA {
    hmap: HashMap<Id, OpCodeHandler>,
}

impl ISA {
    fn new() -> ISA {
        let mut inst = ISA {
            hmap: HashMap::new(),
        };

        inst.register_opcode(
            0x00E,
            OpCodeHandler {
                name: "CLS",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    Some(())
                },
            });

         inst.register_opcode(
             0x00EE,
             OpCodeHandler {
                 name: "RET",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.pc = ctx.stack.pop().unwrap() as usize;
                     Some(())
                 },
             });

         inst.register_opcode(
             0x0000,
             OpCodeHandler {
                 name: "INV",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     println!("invalid");
                     Some(())
                 },
             });
         
         inst.register_opcode(
             0x1000,
             OpCodeHandler {
                 name: "JP",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.pc = to_addr((arg.1, arg.2, arg.3));
                     Some(())
                 },
             });

         inst.register_opcode(
             0xB000,
             OpCodeHandler {
                 name: "JP_V0",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.pc = to_addr((arg.1, arg.2, arg.3)) + ctx.regs[0] as usize;
                     Some(())
                 },
             });

         inst.register_opcode(
             0x2000,
             OpCodeHandler {
                 name: "CALL",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.stack.push(ctx.pc as u16);
                     ctx.pc = to_addr((arg.1, arg.2, arg.3));
                     Some(())
                 },
             });

         inst.register_opcode(
             0x3000,
             OpCodeHandler {
                 name: "SE_BYTE",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     if ctx.regs[arg.1 as usize] == to_u8((arg.2, arg.3)) {
                         ctx.pc += 2;
                     }
                     Some(())
                 },
             });

         inst.register_opcode(
             0x4000,
             OpCodeHandler {
                 name: "SNE_BYTE",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     if ctx.regs[arg.1 as usize] != to_u8((arg.2, arg.3)) {
                         ctx.pc += 2;
                     }
                     Some(())
                 },
             });

         inst.register_opcode(
             0x5000,
             OpCodeHandler {
                 name: "SE_REG",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     if ctx.regs[arg.1 as usize] == ctx.regs[arg.2 as usize] {
                         ctx.pc += 2;
                     }
                     Some(())
                 },
             });

         inst.register_opcode(
             0x6000,
             OpCodeHandler {
                 name: "LD_BYTE",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[arg.1 as usize] = to_u8((arg.2, arg.3));
                     Some(())
                 },
             });

         inst.register_opcode(
             0x7000,
             OpCodeHandler {
                 name: "ADD_BYTE",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[arg.1 as usize] += to_u8((arg.2, arg.3));
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8000,
             OpCodeHandler {
                 name: "LD",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[arg.1 as usize] = ctx.regs[arg.2 as usize];
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8001,
             OpCodeHandler {
                 name: "OR",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[arg.1 as usize] |= ctx.regs[arg.2 as usize];
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8002,
             OpCodeHandler {
                 name: "AND",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[arg.1 as usize] &= ctx.regs[arg.2 as usize];
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8003,
             OpCodeHandler {
                 name: "XOR",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[arg.1 as usize] ^= ctx.regs[arg.2 as usize];
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8004,
             OpCodeHandler {
                 name: "ADD",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     let vx = ctx.regs[arg.1 as usize];
                     let vy = ctx.regs[arg.2 as usize];

                     ctx.regs[VF] = (vx > (std::u8::MAX - vy)) as u8;
                     ctx.regs[arg.1 as usize] += vy;
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8005,
             OpCodeHandler {
                 name: "SUB",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     let vx = ctx.regs[arg.1 as usize];
                     let vy = ctx.regs[arg.2 as usize];

                     ctx.regs[VF] = (vx > vy) as u8;
                     ctx.regs[arg.1 as usize] -= vy;
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8006,
             OpCodeHandler {
                 name: "SHR",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[VF] = ctx.regs[arg.1 as usize] & 0x1;
                     ctx.regs[arg.1 as usize] >>= 1;
                     Some(())
                 },
             });

         inst.register_opcode(
             0x8007,
             OpCodeHandler {
                 name: "SUBN",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     let vx = ctx.regs[arg.1 as usize];
                     let vy = ctx.regs[arg.2 as usize];

                     ctx.regs[VF] = (vy > vx) as u8;
                     ctx.regs[arg.1 as usize] -= vx;
                     Some(())
                 },
             });

         inst.register_opcode(
             0x800E,
             OpCodeHandler {
                 name: "SHL",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.regs[VF] = ctx.regs[arg.1 as usize] >> 7;
                     ctx.regs[arg.1 as usize] <<= 1;
                     Some(())
                 },
             });

         inst.register_opcode(
             0x9000,
             OpCodeHandler {
                 name: "SNE_REG",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     if ctx.regs[arg.1 as usize] == ctx.regs[arg.2 as usize] {
                         ctx.pc += 2;
                     }
                     Some(())
                 },
             });

         inst.register_opcode(
             0xA000,
             OpCodeHandler {
                 name: "LD_I",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.ireg = to_addr((arg.1, arg.2, arg.3));
                     Some(())
                 },
             });

         inst.register_opcode(
             0xB000,
             OpCodeHandler {
                 name: "LD_V0",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.pc = to_addr((arg.1, arg.2, arg.3)) + ctx.regs[0] as usize;
                     Some(())
                 },
             });

         inst.register_opcode(
             0xC000,
             OpCodeHandler {
                 name: "RND",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     let x: u8 = rand::random();
                     ctx.regs[arg.1 as usize] = x & to_u8((arg.2, arg.3));
                     ctx.ireg = to_addr((arg.1, arg.2, arg.3));
                     Some(())
                 },
             });

         inst.register_opcode(
             0xD000,
             OpCodeHandler {
                 name: "DRW",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     Some(())
                 },
             });

         inst.register_opcode(
             0xE09E,
             OpCodeHandler {
                 name: "SKP_VX",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     Some(())
                 },
             });

         inst.register_opcode(
             0xE0A1,
             OpCodeHandler {
                 name: "SKNP_VX",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     Some(())
                 },
             });

         inst.register_opcode(
             0xF007,
             OpCodeHandler {
                 name: "LD_VX_DT",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     Some(())
                 },
             });

         inst.register_opcode(
             0xF00A,
             OpCodeHandler {
                 name: "W_KEY",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     Some(())
                 },
             });
         
         inst.register_opcode(
             0xF015,
             OpCodeHandler {
                 name: "LD_DT_VX",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     Some(())
                 },
             });

         inst.register_opcode(
             0xF018,
             OpCodeHandler {
                 name: "LD_ST_VX",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     Some(())
                 },
             });

         inst.register_opcode(
             0xF01E,
             OpCodeHandler {
                 name: "ADD_I_VX",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.ireg += ctx.regs[arg.1 as usize] as usize;    
                     Some(())
                 },
             });

         inst.register_opcode(
             0xF029,
             OpCodeHandler {
                 name: "LD_F_VX",
                 executor: |ctx: &mut CPU, arg: ArgOctets| {
                     ctx.ireg = ctx.mem_bus.get_sprite_addr(arg.1).unwrap();
                     Some(())
                 },
             });

         inst
    }

    fn register_opcode(&mut self, id: Id, handler: OpCodeHandler) -> &mut Self {
        self.hmap.insert(id, handler);
        self
    }
}

const NUM_GP_REGS: usize = 16;
const PC_START_ADDR: usize = 0x200;
const VF: usize = 0xF;
struct CPU<'a> {
    ireg: usize,
    pc: usize,
    regs: [u8; NUM_GP_REGS],
    delay_reg: u8,
    timer_reg: u8,
    stack: Stack,
    isa: ISA,
    mem_bus: &'a (MemoryBus +  'a), 
}

impl<'a> CPU<'a>
{
    fn new(mem_bus: &'a MemoryBus) -> CPU<'a> {
        CPU
        {
            ireg: 0,
            pc: PC_START_ADDR,
            regs: [0; NUM_GP_REGS],
            delay_reg: 0,
            timer_reg: 0,
            stack: Stack::new(),
            isa: ISA::new(),
            mem_bus: mem_bus,
        }
    }
}

trait PipeLine {
    fn fetch(&mut self) -> Option<u16>;
    fn decode(&self, instruction: u16) -> Option<(Id, ArgOctets)>;
    fn execute(&mut self, id: Id, arg: ArgOctets) -> Option<()>;
}

impl<'a> PipeLine for CPU<'a>
{
    fn fetch(&mut self) -> Option<u16> {        
        let cur_inst = self.mem_bus.get_instruction(self.pc).unwrap() as u16;
        self.pc += 2;
        println!("fetched instruction: {:04X}", cur_inst);
        Some(cur_inst as u16)
    }

    fn decode(&self, instruction: u16) -> Option<(Id, ArgOctets)> {
        let octs = to_octets(instruction);

        let id = match octs {
            (0x0, _, _, _) => to_id((0x0, 0x0, octs.2, octs.3)),

            (0x1, _, _, _) | (0x2, _, _, _) | (0x3, _, _, _) |
            (0x4, _, _, _) | (0x5, _, _, _) | (0x6, _, _, _) |
            (0x7, _, _, _) | (0xA, _, _, _) | (0xB, _, _, _) |
            (0xC, _, _, _) | (0xD, _, _, _) | (0x9, _, _, _) => to_id((octs.0, 0x0, 0x0, 0x0)),

            (0x8, _, _, _) => to_id((octs.0, 0x0, 0x0, octs.3)),
            (0xE, _, _, _) | (0xF, _, _, _) => to_id((octs.0, 0x0, octs.2, octs.3)),

            (_, _, _, _) => 0,
        };

        println!("decoded instruction: {:04X} => opcode {:04x}, arg {:?}",
                 instruction, id, octs);
        Some((id, octs)) 
    }

    fn execute(&mut self, id: Id, arg: ArgOctets) -> Option<()>{
        {
            let h = self.isa.hmap.get(&id).unwrap();
            println!("execute opcode: {:04X} => {} with arg {:?}", id, h.name, arg);
        }
        (self.isa.hmap[&id].executor)(self, arg)
    }
}

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

    println!("exe len {}", exe.len());

    exe
}

fn execute_cycle<P: PipeLine>(pl: &mut P) -> Option<()> {
    for _ in 0..32 {
        let instruction = pl.fetch().unwrap();
        let (id, arg) = pl.decode(instruction).unwrap();
        let _ = pl.execute(id, arg).unwrap();
    }

    Some(())
}

fn main() {
    let exe = load_game("/home/kobein/evo/rust/chip8_opcode/res/INVADERS".to_string());

    let mem = Memory::new()
        .load_sprites(SPRITES)
        .load_exe(exe.as_slice())
        .build();

    let mut emulator = CPU::new(&mem as &MemoryBus);
    execute_cycle(&mut emulator);
}
