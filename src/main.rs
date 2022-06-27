struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;
        op_byte1 << 8 | op_byte2
    }

    fn run (&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;
            let c = ((opcode & 0xF000) >> 12) as u8; 
            let x = ((opcode & 0x0F00) >>  8) as u8; 
            let y = ((opcode & 0x00F0) >>  4) as u8; 
            let d = ((opcode & 0x000F) >>  0) as u8; 
            let nnn = opcode & 0x0FFFF;
            // let kk = (opcode & 0x00FF) as u8

            match (c, x, y, d) {
                (0, 0, 0, 0)        => {return;},
                (0, 0, 0xE, 0xE)    => self.ret(),
                (0x2, _, _, _)      => self.call(nnn),
                (0x8, _, _, 0x4)    => self.add_xy(x, y),
                _                   => todo!("opcode {:04x}", opcode)
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;
        
        if sp > stack.len() {
            panic!("Stack overflow!");
        }
    }
}

fn main() {}
