
// stack mod
struct Stack<'a> {
    mem: &'a mut [u16],
    top: isize,
    size: usize,
}

impl<'a> Stack<'a> {
    fn new(mem: &'a mut [u16], size: usize) -> Self {
        Stack {
            mem: mem,
            top: -1,
            size: size,
        }
    }

    fn push(&mut self, val: u16) -> Option<()> {
        if self.size as isize - self.top == 0 {
            return None;
        }
        
        self.top += 1;
        self.mem[self.top as usize] = val;

        Some(())
    }

    fn pop(&mut self) -> Option<u16> {
        if self.top < 0 {
            return None;
        }
        
        let val = self.mem[self.top as usize];
        self.top -= 1;

        Some(val)
    }
}

struct MemoryView<'a> {
    rom: &'a [u16],
    program: &'a [u16],
}

impl<'a> MemoryView<'a> {

    fn new(rom_ref: &'a [u16], exe_ref: &'a [u16] ) -> MemoryView<'a> {
        MemoryView {
            rom: rom_ref,
            program: exe_ref,
        }
    }

    fn get_instruction(&self, pc_addr: usize) -> Option<&u16> {
        self.program.get(pc_addr)
    }
}

const NUM_GP_REGS: usize = 16;
const PC_START_ADDR: u16 = 0x200;
struct HwContext<'a> {
    i_reg: u16,
    pc: u16,
    regs: [u8; NUM_GP_REGS],
    delay_reg: u8,
    timer_reg: u8,
    stack: Stack<'a>,
}

impl<'a> HwContext<'a> {
    fn new(stack: Stack<'a> ) -> Self {
        HwContext {
            i_reg: 0,
            pc: PC_START_ADDR,
            regs: [0; NUM_GP_REGS],
            delay_reg: 0,
            timer_reg: 0,
            stack: stack,
        }
    }
}

const ROM_BEGIN_ADDR: usize = 0;
const ROM_END_ADDR: usize = 0x1FF;
const EXE_BEGIN_ADDR: usize = 0x200;
const EXE_END_ADDR: usize = 0xFFF;
const MEM_SIZE: usize = 0xFFF;
const STACK_SIZE: usize = 16;
struct Emulator<'a> {
    mem_view: MemoryView<'a>,
    hw_ctx: HwContext<'a>,
}

impl<'a> Emulator<'a> {
    fn new(mem: &'a [u16], stack: &'a mut [u16], stack_sz: usize) -> Self {        
        Emulator {
            mem_view: MemoryView::new(&mem[ROM_BEGIN_ADDR..ROM_END_ADDR],
                                      &mem[EXE_BEGIN_ADDR..EXE_END_ADDR]),
            hw_ctx: HwContext::new(Stack::new(stack, stack_sz)),
        }
    }
}

enum OpCode{ClearDisplay}

trait PipeLine {
    fn fetch(&self) -> Option<&u16>;
    fn decode(&self, instr: u16) -> Option<(OpCode, u16)>;
    fn execute(&self, icode: OpCode, arg: u16) -> Option<()>;
}

impl<'a> PipeLine for Emulator<'a> {
    fn fetch(&self) -> Option<&u16> {
        self.mem_view.get_instruction(self.hw_ctx.pc as usize)
    }

    fn decode(&self, instr: u16) -> Option<(OpCode, u16)> {
        Some((OpCode::ClearDisplay, 0))
    }

    fn execute(&self, icode: OpCode, arg: u16) -> Option<()>{
        Some(())
    }
}
//impl PipeLine for HwContext {
//    fn fetch(&self) -> Option<u16> {
//    }
//    fn decode(&self, instr: u16) -> Option<(OpCode, u16)>;
//    fn execute(&self, (icode: OpCode, arg: u16)) -> Option<()>;
//}

fn main() {
    let mut main_mem: [u16; MEM_SIZE] = [0; MEM_SIZE];
    let mut stack_mem: [u16; STACK_SIZE] = [0; STACK_SIZE];

    let mut emulator = Emulator::new(&main_mem, &mut stack_mem, STACK_SIZE);
    println!("instr: {}", emulator.fetch().unwrap());
}
