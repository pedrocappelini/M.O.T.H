use crate::vm2_bits::Vm2Bits;

//todo: make the registers work properly.

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
                //halt
            }
            //using a+b as A_high + B-high, and the same with LOW, passing the carry to the High or in to another register in case of overflow.
            //self.reg[0] will store the "low_bytes" of the result and self.alu.reg[0] will store the "high_bytes".
            0b0001 => {
                let a_high = nibble1 >> 2;
                let a_low = nibble1 & 0b11;
                let b_high = nibble2 >> 2;
                let b_low = nibble2 & 0b11;

                self.alu.reg[0] = a_low;
                self.alu.reg[1] = b_low;
                self.alu.cycles();
                self.reg[0] = self.alu.reg[0];

                if self.alu.reg[3] == 1 {
                    self.alu.adc = true;
                }

                self.alu.reg[0] = a_high;
                self.alu.reg[1] = b_high;
                self.alu.cycles();

                self.reg[1] = self.reg[0] | self.alu.reg[0] << 2;
                self.reg[3] = self.alu.reg[3]; // carry from the 2nd sum. Will be needed for the 8bits

                println!("{:04b}", self.reg[1]);
                println!("{:04b}", self.reg[3]);
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
