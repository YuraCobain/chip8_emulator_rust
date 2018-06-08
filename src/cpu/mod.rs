extern crate rand;

use memory::*;
use cpu_ops::*;
use media_if::*;
use utils::*;

use std::collections::HashMap;

struct OpCodeHandler<'a> {
    name: &'static str,
    executor: fn(&mut CPU<'a>, ArgOctets) -> Option<()>, 
}

struct ISA<'a> {
    hmap: HashMap<Id, OpCodeHandler<'a>>,
}

impl<'a> ISA<'a> {
    fn new() -> ISA<'a> {
        ISA {
            hmap: HashMap::new(),
        }
    }

    fn register_opcode(&mut self, id: Id, handler: OpCodeHandler<'a>) -> &mut Self {
        self.hmap.insert(id, handler);
        self
    }
}

const NUM_GP_REGS: usize = 16;
const PC_START_ADDR: u16 = 0x200;
const VF: usize = 0xF;
pub struct CPU<'a> {
    ireg: u16,
    pc: u16,
    regs: [u8; NUM_GP_REGS],
    delay_reg: u8,
    sound_reg: u8,
    isa: ISA<'a>,
    cpu_mem: &'a mut (CpuMemory + 'a), 
    gfx_mem: &'a mut (VideoMemory + 'a), 
    media_if: &'a mut (MediaIf + 'a),
}

impl<'a> CPU<'a>
{
    pub fn new(cpu_mem: &'a mut CpuMemory,
           gfx_mem: &'a mut VideoMemory,
           media_if: &'a mut MediaIf) -> CPU<'a> {
        let mut cpu = CPU
        {
            ireg: 0,
            pc: PC_START_ADDR,
            regs: [0; NUM_GP_REGS],
            delay_reg: 0,
            sound_reg: 0,
            isa: ISA::new(),
            cpu_mem: cpu_mem,
            gfx_mem: gfx_mem,
            media_if: media_if,
        };

        cpu.isa.register_opcode(
            0x0000,
            OpCodeHandler {
                name: "INV",
                executor: |_ctx: &mut CPU, _arg: ArgOctets| {
                    println!("invalid");
                    Some(())
                },
            });

        cpu.isa.register_opcode( 
            0x00E0,
            OpCodeHandler { 
                name: "CLS",
                executor: |ctx: &mut CPU, _arg: ArgOctets| {
                    ctx.gfx_mem.clear();
                    ctx.media_if.clear_display();
                    ctx.media_if.present_display();
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x00EE,
            OpCodeHandler {
                name: "RET",
                executor: |ctx: &mut CPU, _arg: ArgOctets| {
                    ctx.pc = ctx.cpu_mem.pop().unwrap();
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x1000,
            OpCodeHandler {
                name: "JP",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.pc = to_addr((arg.1, arg.2, arg.3));
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x2000,
            OpCodeHandler {
                name: "CALL",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.cpu_mem.push(ctx.pc as u16);
                    ctx.pc = to_addr((arg.1, arg.2, arg.3));
                    Some(())
                },
            });

        cpu.isa.register_opcode(
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

        cpu.isa.register_opcode(
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

        cpu.isa.register_opcode(
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

        cpu.isa.register_opcode(
            0x6000,
            OpCodeHandler {
                name: "LD_BYTE",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[arg.1 as usize] = to_u8((arg.2, arg.3));
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x7000,
            OpCodeHandler {
                name: "ADD_BYTE",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let vx = ctx.regs[arg.1 as usize];
                    let vy = to_u8((arg.2, arg.3));
                    let res = vx.overflowing_add(vy);

                    ctx.regs[arg.1 as usize] = res.0;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8000,
            OpCodeHandler {
                name: "LD",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[arg.1 as usize] = ctx.regs[arg.2 as usize];
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8001,
            OpCodeHandler {
                name: "OR",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[arg.1 as usize] |= ctx.regs[arg.2 as usize];
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8002,
            OpCodeHandler {
                name: "AND",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[arg.1 as usize] &= ctx.regs[arg.2 as usize];
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8003,
            OpCodeHandler {
                name: "XOR",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[arg.1 as usize] ^= ctx.regs[arg.2 as usize];
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8004,
            OpCodeHandler {
                name: "ADD",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let vx = ctx.regs[arg.1 as usize];
                    let vy = ctx.regs[arg.2 as usize];
                    let res = vx.overflowing_add(vy);

                    ctx.regs[arg.1 as usize] = res.0;
                    ctx.regs[VF] = res.1 as u8;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8005,
            OpCodeHandler {
                name: "SUB",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let vx = ctx.regs[arg.1 as usize];
                    let vy = ctx.regs[arg.2 as usize];
                    let res = vx.overflowing_sub(vy);

                    ctx.regs[arg.1 as usize] = res.0;
                    ctx.regs[VF] = !res.1 as u8;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8006,
            OpCodeHandler {
                name: "SHR",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[VF] = ctx.regs[arg.1 as usize] & 0x1;
                    ctx.regs[arg.1 as usize] >>= 1;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x8007,
            OpCodeHandler {
                name: "SUBN",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let vx = ctx.regs[arg.1 as usize];
                    let vy = ctx.regs[arg.2 as usize];
                    let res = vy.overflowing_sub(vx);

                    ctx.regs[arg.1 as usize] = res.0;
                    ctx.regs[VF] = !res.1 as u8;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0x800E,
            OpCodeHandler {
                name: "SHL",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[VF] = ctx.regs[arg.1 as usize] >> 7;
                    ctx.regs[arg.1 as usize] <<= 1;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
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

        cpu.isa.register_opcode(
            0xA000,
            OpCodeHandler {
                name: "LD_I",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.ireg = to_addr((arg.1, arg.2, arg.3));
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xB000,
            OpCodeHandler {
                name: "LD_V0",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.pc = to_addr((arg.1, arg.2, arg.3)) + ctx.regs[0] as u16;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xC000,
            OpCodeHandler {
                name: "RND",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let x: u8 = rand::random();
                    ctx.regs[arg.1 as usize] = x & to_u8((arg.2, arg.3));
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xD000,
            OpCodeHandler {
                name: "DRW",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let x = ctx.regs[arg.1 as usize];
                    let y = ctx.regs[arg.2 as usize];
                    let sprites = ctx.cpu_mem.get_sprites(ctx.ireg, arg.3).unwrap();

                    ctx.regs[VF] = ctx.gfx_mem.apply_sprites(x, y, sprites).unwrap();
                    ctx.media_if.clear_display();
                    ctx.media_if.draw_display(ctx.gfx_mem.get_video_buf().unwrap());
                    ctx.media_if.present_display();
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xE09E,
            OpCodeHandler {
                name: "SKP_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    if ctx.media_if.is_key_pressed(ctx.regs[arg.1 as usize]) {
                        ctx.pc += 2;
                    }
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xE0A1,
            OpCodeHandler {
                name: "SKNP_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    if !ctx.media_if.is_key_pressed(ctx.regs[arg.1 as usize]) {
                        ctx.pc += 2;
                    }
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF007,
            OpCodeHandler {
                name: "LD_VX_DT",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.regs[arg.1 as usize] = ctx.delay_reg;
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF00A,
            OpCodeHandler {
                name: "W_KEY",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let mut key = 20;
                    for i in 0..16 {
                        if ctx.media_if.is_key_pressed(i) {
                            key = i;
                        }
                    }

                    if key == 20 {
                        ctx.pc -= 2;
                    } else {
                        ctx.regs[arg.1 as usize] = key as u8;
                    }
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF015,
            OpCodeHandler {
                name: "LD_DT_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.delay_reg = ctx.regs[arg.1 as usize];
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF018,
            OpCodeHandler {
                name: "LD_ST_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.sound_reg = ctx.regs[arg.1 as usize];
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF01E,
            OpCodeHandler {
                name: "ADD_I_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    ctx.ireg += ctx.regs[arg.1 as usize] as u16;    
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF029,
            OpCodeHandler {
                name: "LD_F_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    println!("sprite for font {}", ctx.regs[arg.1 as usize]);
                    ctx.ireg = ctx.cpu_mem.get_font_sprite_addr(ctx.regs[arg.1 as usize]).unwrap();
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF033,
            OpCodeHandler {
                name: "LD_B_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    let mut x = ctx.regs[arg.1 as usize];

                    for i in (0..3).rev() {
                        ctx.cpu_mem.set_u8(ctx.ireg + i, x % 10);
                        x /= 10;
                    }
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF055,
            OpCodeHandler {
                name: "LD_I_VX",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    for i in 0..=arg.1 as u16 {
                        ctx.cpu_mem.set_u8(ctx.ireg + i, ctx.regs[i as usize]);
                    }
                    Some(())
                },
            });

        cpu.isa.register_opcode(
            0xF065,
            OpCodeHandler {
                name: "LD_VX_I",
                executor: |ctx: &mut CPU, arg: ArgOctets| {
                    for i in 0..=arg.1 as u16 {
                        ctx.regs[i as usize] = ctx.cpu_mem.get_u8(ctx.ireg + i).unwrap();
                    }
                    Some(())
                },
            });

        cpu
    }
}

impl<'a> PipeLine for CPU<'a>
{
    fn process_events(&mut self) -> bool {
        self.media_if.process_events()
    }

    fn fetch(&mut self) -> Option<u16> {        
        let cur_inst = self.cpu_mem.get_instruction(self.pc).unwrap() as u16;
        self.pc += 2;

        //println!("fetched instruction: {:04X}", cur_inst);
        Some(cur_inst as u16)
    }

    fn decode(&self, instruction: u16) -> Option<(Id, ArgOctets)> {
        let octs = to_octets(instruction);

        let id = match octs {
            (0x0, _, _, _) => to_id((0x0, 0x0, octs.2, octs.3)),
            (0x1, _, _, _) |
            (0x2, _, _, _) | 
            (0x3, _, _, _) |
            (0x4, _, _, _) | 
            (0x5, _, _, _) | 
            (0x6, _, _, _) |
            (0x7, _, _, _) | 
            (0xA, _, _, _) | 
            (0xB, _, _, _) |
            (0xC, _, _, _) | 
            (0xD, _, _, _) | 
            (0x9, _, _, _) => to_id((octs.0, 0x0, 0x0, 0x0)),
            (0x8, _, _, _) => to_id((octs.0, 0x0, 0x0, octs.3)),
            (0xE, _, _, _) |
            (0xF, _, _, _) => to_id((octs.0, 0x0, octs.2, octs.3)),

            (_, _, _, _) => 0,
        };

        //println!("decoded instruction: {:04X} => opcode {:04x}, arg {:?}",
        //         instruction, id, octs);
        Some((id, octs)) 
    }

    fn execute(&mut self, id: Id, arg: ArgOctets) -> Option<()>{
        {
            let h = self.isa.hmap.get(&id).unwrap();
            println!("execute opcode: {:04X} => {} with arg {:?}", id, h.name, arg);
        }
        (self.isa.hmap[&id].executor)(self, arg);

        Some(())
    }

    fn update_timers(&mut self) {
        self.delay_reg = self.delay_reg.saturating_sub(1);
        self.sound_reg = self.sound_reg.saturating_sub(1);
    }
}
