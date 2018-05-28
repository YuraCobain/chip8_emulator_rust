mod stack;

use stack::*;
use std::collections::HashMap;

const NUM_GP_REGS: usize = 16;
const PC_START_ADDR: u16 = 0x200;
const ROM_BEGIN_ADDR: usize = 0;
const ROM_END_ADDR: usize = 0x1FF;
const EXE_BEGIN_ADDR: usize = 0x200;
const EXE_END_ADDR: usize = 0xFFF;
const MEM_SIZE: usize = 0xFFF;

struct Memory {
    memory: [u8; MEM_SIZE],
}

impl Memory {
    fn new() -> Self {
        Memory {
            memory: [0; MEM_SIZE],
        }
    }

    fn rom_slice(&self) -> &[u8] {
        &self.memory[ROM_BEGIN_ADDR..ROM_END_ADDR]
    }

    fn exe_slice(&self) -> &[u8] {
        &self.memory[EXE_BEGIN_ADDR..EXE_END_ADDR] 
    }

    fn get_instruction(&self, addr: u16) -> u16 {
        let high_byte: u16 = self.memory[addr as usize] as u16;
        let low_byte: u16 = self.memory[(addr + 1) as usize] as u16;

        high_byte << 8 | low_byte
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum InstructionCode{
    CLS,
    RET,
    SYS,
    JP,
    CALL,
    SE_BYTE, 
    SNE_REG,
    SE_REG,  
    LD_BYTE,
    ADD_REG,
    LD_REG,  //----
    OR,
    AND,
    XOR,
    ADD,
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
    Some(())
}

fn sys(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("sys");
    Some(())
}

fn jp(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("jp");
    Some(())
}

fn call(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("call");
    Some(())
}

fn se_byte(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("se_byte");
    Some(())
}

fn sne_reg(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("sne");
    Some(())
}

fn se_reg(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("se_reg");
    Some(())
}

fn ld_byte(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("ld_byte");
    Some(())
}

fn add_reg(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("add_reg");
    Some(())
}

fn ld_reg(ctx: &mut CPU, arg: ArgOctets) -> Option<()> {
    println!("ld_reg");
    Some(())
}

type ArgOctets = (u8, u8, u8, u8);
type InstructionExecutor = fn(ctx: &mut CPU, arg: ArgOctets) -> Option<()>;

struct InstructionDetails {
    executor: InstructionExecutor,
}

struct ISA {
    isa_map: HashMap<InstructionCode, InstructionDetails>,
}

impl ISA {
    fn new() -> ISA {
        let mut isa = ISA {
            isa_map: HashMap::new(),
        };

        isa.isa_map.insert(InstructionCode::CLS,     InstructionDetails { executor: clr});
        isa.isa_map.insert(InstructionCode::RET,     InstructionDetails { executor: ret});
        isa.isa_map.insert(InstructionCode::SYS,     InstructionDetails { executor: sys});
        isa.isa_map.insert(InstructionCode::JP,      InstructionDetails { executor: jp});
        isa.isa_map.insert(InstructionCode::CALL,    InstructionDetails { executor: call});
        isa.isa_map.insert(InstructionCode::SE_BYTE, InstructionDetails { executor: se_byte});
        isa.isa_map.insert(InstructionCode::SNE_REG, InstructionDetails { executor: sne_reg});
        isa.isa_map.insert(InstructionCode::SE_REG,  InstructionDetails { executor: se_reg});
        isa.isa_map.insert(InstructionCode::LD_BYTE, InstructionDetails { executor: ld_byte});
        isa.isa_map.insert(InstructionCode::ADD_REG, InstructionDetails { executor: add_reg});
        isa.isa_map.insert(InstructionCode::LD_REG,  InstructionDetails { executor: ld_reg});

        isa
    }
}

struct CPU {
    i_reg: u16,
    pc: u16,
    regs: [u8; NUM_GP_REGS],
    delay_reg: u8,
    timer_reg: u8,
    stack: Stack,
    isa: ISA,
}

impl CPU
{
    fn new() -> Self {
        CPU
        {
            i_reg: 0,
            pc: PC_START_ADDR,
            regs: [0; NUM_GP_REGS],
            delay_reg: 0,
            timer_reg: 0,
            stack: Stack::new(),
            isa: ISA::new(),
        }
    }
}

struct Emulator {
    cpu: CPU,
    memory: Memory, 
}

impl Emulator {
    fn new() -> Self {
         Emulator {
            cpu: CPU::new(),
            memory: Memory::new(),
        }
    }

    fn execute_cycle(&mut self) -> Option<()> {
        let instruction = self.cpu.fetch(&self.memory).unwrap();
        let (icode, arg) : (InstructionCode, ArgOctets) = self.cpu.decode(instruction).unwrap();
        let _ = self.cpu.execute(icode, arg).unwrap();

        Some(())
    }
}

fn get_octet(x: u16, num: u16) -> u8 {
    (x & (0xF << (num << 2)) >> (num >> 2)) as u8
}

fn to_octets(x: u16) -> (u8, u8, u8, u8) {
    let oct0 = get_octet(x, 0);
    let oct1 = get_octet(x, 1);
    let oct2 = get_octet(x, 2);
    let oct3 = get_octet(x, 3);

    (oct0, oct1, oct2, oct3)
}

trait PipeLine {
    fn fetch(&self, mem: &Memory) -> Option<u16>;
    fn decode(&self, instruction: u16) -> Option<(InstructionCode, ArgOctets)>;
    fn execute(&mut self, icode: InstructionCode, arg: ArgOctets) -> Option<()>;
}

impl PipeLine for CPU
{
    fn fetch(&self, mem: &Memory) -> Option<u16> {
        Some(mem.get_instruction(self.pc))
    }

    fn decode(&self, instruction: u16) -> Option<(InstructionCode, ArgOctets)> {
        let octets = to_octets(instruction);

        let icode = match octets {
            (0x0, 0x0, 0xE, 0x0) => InstructionCode::CLS,
            (0x0, 0x0, 0xE, 0xE) => InstructionCode::RET,
            (0x0, _, _, _) =>  InstructionCode::SYS,
            (0x1, _, _, _) =>  InstructionCode::JP,
            (0x2, _, _, _) =>  InstructionCode::CALL,
            (0x3, _, _, _) =>  InstructionCode::SE_BYTE,
            (0x4, _, _, _) =>  InstructionCode::SNE_REG,
            (0x5, _, _, _) =>  InstructionCode::SE_REG,
            (0x6, _, _, _) =>  InstructionCode::LD_BYTE,
            (0x7, _, _, _) =>  InstructionCode::ADD_REG,
            (0x8, _, _, 0x0) =>  InstructionCode::LD_REG,
            (_, _, _, _) => InstructionCode::CLS,
        };

        Some((icode, octets)) 
    }

    fn execute(&mut self, icode: InstructionCode, arg: ArgOctets) -> Option<()>{
        (self.isa.isa_map[&icode].executor)(self, arg)
    }

}

use std::io;
use std::io::prelude::*;
use std::fs::File;

fn load_game(path: &'static str) -> Memory {
    let mut f = File::open("foo.txt").unwrap();
    let mut bin = [0; MEM_SIZE];

    f.read(&mut bin).unwrap();

    Memory {
        memory: bin,
    }
}

fn main() {
    let mut emulator = Emulator::new();

    emulator.execute_cycle();
}
