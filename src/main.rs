mod stack;
mod memory;
mod sprites;

use stack::*;
use memory::*;
use sprites::*;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum OpCode{
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE_BYTE, 
    SNE_BYTE,
    SE_REG,  
    SNE_REG,  
    LD_BYTE,
    ADD_BYTE,
    LD_REG,  //----
    OR,
    AND,
    XOR,
    ADD_REG,
    SUB,
    SHR,
    SUBN,
    SHL,
    SNE,
    LD_I_A,
    JP_REG,
    RND,
    DRW,
    SKP,
    SKNP,
    LD_1,
    LD_2,
    LD_3,
    LD_4,
    ADD_I,
    LD_5,
    LD_6,
    LD_7,
    LD_8,
}

fn clr(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("clr");
    Some(())
}

fn ret(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("ret");
    ctx.pc = ctx.stack.pop().unwrap();
    Some(())
}

fn sys(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("sys");
    Some(())
}

fn jp(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("jp");
    ctx.pc = to_addr((arg.1, arg.2, arg.3));
    Some(())
}

fn call(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.stack.push(ctx.pc);
    ctx.pc = to_addr((arg.1, arg.2, arg.3));
    println!("call: {:?} on addr {:x}", arg, ctx.pc);
    Some(())
}

fn se_byte(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    if ctx.regs[arg.1 as usize] == to_u8((arg.2, arg.3)) {
        ctx.pc += 2;
    }
    println!("se_byte");
    Some(())
}

fn sne_byte(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    if ctx.regs[arg.1 as usize] != to_u8((arg.2, arg.3)) {
        ctx.pc += 2;
    }
    println!("sne_byte");
    Some(())
}

fn se_reg(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    if ctx.regs[arg.1 as usize] == ctx.regs[arg.2 as usize] {
        ctx.pc += 2;
    }
    println!("se_reg");
    Some(())
}

fn sne_reg(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    if ctx.regs[arg.1 as usize] != ctx.regs[arg.2 as usize] {
        ctx.pc += 2;
    }
    println!("se_reg");
    Some(())
}

fn ld_byte(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("ld_byte");
    ctx.regs[arg.1 as usize] = to_u8((arg.2, arg.3));
    Some(())
}

fn add_byte(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[arg.1 as usize] += to_u8((arg.2, arg.3));
    println!("add_byte");
    Some(())
}

fn ld_reg(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[arg.1 as usize] += ctx.regs[arg.2 as usize];
    println!("ld_reg");
    Some(())
}

fn or(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[arg.1 as usize] |= ctx.regs[arg.2 as usize];
    println!("or");
    Some(())
}

fn and(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[arg.1 as usize] &= ctx.regs[arg.2 as usize];
    println!("and");
    Some(())
}

fn xor(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[arg.1 as usize] ^= ctx.regs[arg.2 as usize];
    println!("xor");
    Some(())
}

fn add(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[0xF] = (ctx.regs[arg.1 as usize] > (0xFF - ctx.regs[arg.2 as usize])) as u8;
    ctx.regs[arg.1 as usize] += ctx.regs[arg.2 as usize];
    println!("add");
    Some(())
}

fn sub(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[0xF] = (ctx.regs[arg.1 as usize] > ctx.regs[arg.2 as usize]) as u8;
    ctx.regs[arg.1 as usize] += ctx.regs[arg.2 as usize];
    println!("sub");
    Some(())
}

fn shr(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    ctx.regs[0xF] = (arg.1 >> 7);
    println!("shr");
    Some(())
}

fn subn(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("subn");
    Some(())
}

fn shl(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("shl");
    Some(())
}

fn sne(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("she");
    Some(())
}

fn ld_i_a(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("ld_i_a");
    ctx.i_reg = to_addr((arg.1, arg.2, arg.3));
    Some(())
}
type ArgOctets = (u8, u8, u8, u8);
type OpCodeExe = fn(ctx: &mut CPU, arg: ArgOctets) -> Option<()>;

struct OpCodeHandler {
    executor: OpCodeExe,
}

struct ISA {
    isa_map: HashMap<OpCode, OpCodeHandler>,
}

impl ISA {
    fn new() -> ISA {
        let mut isa = ISA {
            isa_map: HashMap::new(),
        };

        isa.isa_map.insert(OpCode::CLS,      OpCodeHandler { executor: clr});
        isa.isa_map.insert(OpCode::RET,      OpCodeHandler { executor: ret});
        isa.isa_map.insert(OpCode::SYS,      OpCodeHandler { executor: sys});
        isa.isa_map.insert(OpCode::JP,       OpCodeHandler { executor: jp});
        isa.isa_map.insert(OpCode::CALL,     OpCodeHandler { executor: call});
        isa.isa_map.insert(OpCode::SE_BYTE,  OpCodeHandler { executor: se_byte});
        isa.isa_map.insert(OpCode::SNE_BYTE, OpCodeHandler { executor: sne_byte});
        isa.isa_map.insert(OpCode::SE_REG,   OpCodeHandler { executor: se_reg});
        isa.isa_map.insert(OpCode::SNE_REG,  OpCodeHandler { executor: sne_reg});
        isa.isa_map.insert(OpCode::LD_BYTE,  OpCodeHandler { executor: ld_byte});
        isa.isa_map.insert(OpCode::ADD_BYTE, OpCodeHandler { executor: add_byte});
        isa.isa_map.insert(OpCode::LD_REG,   OpCodeHandler { executor: ld_reg});
        isa.isa_map.insert(OpCode::LD_I_A,   OpCodeHandler { executor: ld_i_a});

        isa
    }
}

const NUM_GP_REGS: usize = 16;
const PC_START_ADDR: u16 = 0x200;
struct CPU<'a> {
    i_reg: u16,
    pc: u16,
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
            i_reg: 0,
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

fn get_octet(x: u16, num: u16) -> u8 {
    ((x & (0xF << num)) >> num) as u8
}

fn to_octets(x: u16) -> (u8, u8, u8, u8) {
    let oct0 = get_octet(x, 0);
    let oct1 = get_octet(x, 4);
    let oct2 = get_octet(x, 8);
    let oct3 = get_octet(x, 12);

    (oct3, oct2, oct1, oct0)
}

fn to_addr((oct0, oct1, oct2) : (u8, u8, u8)) -> u16 {
    (oct0 as u16) << 8 | (oct1 as u16) << 4 | (oct2 as u16)
}

fn to_u8((oct0, oct1): (u8, u8)) -> u8 {
    (oct0 << 4) | oct1
}

trait PipeLine {
    fn fetch(&mut self) -> Option<u16>;
    fn decode(&self, instruction: u16) -> Option<(OpCode, ArgOctets)>;
    fn execute(&mut self, icode: OpCode, arg: ArgOctets) -> Option<()>;
}

impl<'a> PipeLine for CPU<'a>
{
    fn fetch(&mut self) -> Option<u16> {        
        let curr_pc = self.pc;
        self.pc += 2;
        Some(self.mem_bus.get_instruction(curr_pc).unwrap())
    }

    fn decode(&self, instruction: u16) -> Option<(OpCode, ArgOctets)> {
        let octets = to_octets(instruction);

        println!("fetched instruction {:04x} -> {:?}", instruction, octets);
        let icode = match octets {
            (0x0, 0x0, 0xE, 0x0) => OpCode::CLS,
            (0x0, 0x0, 0xE, 0xE) => OpCode::RET,
            (0x0, _, _, _) =>  OpCode::SYS,
            (0x1, _, _, _) =>  OpCode::JP,
            (0x2, _, _, _) =>  OpCode::CALL,
            (0x3, _, _, _) =>  OpCode::SE_BYTE,
            (0x4, _, _, _) =>  OpCode::SNE_BYTE,
            (0x5, _, _, _) =>  OpCode::SE_REG,
            (0x6, _, _, _) =>  OpCode::LD_BYTE,
            (0x7, _, _, _) =>  OpCode::ADD_BYTE,
            (0x8, _, _, 0x0) =>  OpCode::LD_REG,
            (0xA, _, _, _) =>  OpCode::LD_I_A,
            (_, _, _, _) => OpCode::CLS,
        };

        Some((icode, octets)) 
    }

    fn execute(&mut self, icode: OpCode, arg: ArgOctets) -> Option<()>{
        (self.isa.isa_map[&icode].executor)(self, arg)
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

fn execute_cycle<T: PipeLine>(emu: &mut T) -> Option<()> {
    for i in 0..10 {
        let instruction = emu.fetch().unwrap();
        let (icode, arg) : (OpCode, ArgOctets) = emu.decode(instruction).unwrap();
        let _ = emu.execute(icode, arg).unwrap();
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
