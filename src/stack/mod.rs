const STACK_SIZE: usize = 16;
pub struct Stack {
    mem: [u16; STACK_SIZE],
    top: isize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            mem: [0; STACK_SIZE],
            top: -1,
        }
    }

    pub fn push(&mut self, val: u16) -> Option<()> {
        if STACK_SIZE as isize - self.top == 0 {
            return None;
        }

        self.top += 1;
        self.mem[self.top as usize] = val;

        Some(())
    }

    pub fn pop(&mut self) -> Option<u16> {
        if self.top < 0 {
            return None;
        }

        let val = self.mem[self.top as usize];
        self.top -= 1;

        Some(val)
    }
}
