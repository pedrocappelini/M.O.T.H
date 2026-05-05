use crate::vm2_bits::Vm2Bits;

pub struct Vm4Bits {
    pub memory: [u8; 256], //0x11111111
    pub reg: [u8; 8],
    pub stack: [u8; 8],
    pub pc: u8,
    pub index: u8,
    pub sp: u8,
    pub alu: Vm2Bits,
}

impl Vm4Bits {
    pub fn new() -> Self {
        Self {
            memory: [0u8; 256],
            reg: [0; 8],
            stack: [0; 8],
            pc: 0,
            index: 0,
            sp: 0,
            alu: Vm2Bits::new(),
        }
    }

    pub fn cycle(&mut self) {
        let instruction = self.memory[self.pc as usize];
        let nibble1 = self.memory[(self.pc + 1) as usize];
        let nibble2 = self.memory[(self.pc + 2) as usize];
        self.pc += 3;

        match instruction {
            0b0000 => {
                println!("halt");
            }
            0b0001 => {
                println!("sum nibble 1 + 2");
            }
            0b0010 => {
                println!("subtract nibble 1 + 2");
            }
            _ => {
                println!("nada")
            }
        }
    }
}
