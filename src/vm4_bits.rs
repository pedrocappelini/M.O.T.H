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
        //todo: make it wrap arround the memory, resetig the cycle.
        let instruction = self.memory[self.pc as usize];
        let nibble1 = self.memory[(self.pc + 1) as usize];
        let nibble2 = self.memory[(self.pc + 2) as usize];
        self.pc += 3;

        match instruction {
            //halt
            0b0000 => {}
            //assigns a value to a register
            0b0010 => {
                let reg_adr = nibble1;
                self.reg[reg_adr as usize] = nibble2;
            }
            //ADD r$, x
            0b0001 => {
                let reg_adr = nibble1;
                let reg_num_low = self.reg[reg_adr as usize] & 0b11;
                let reg_num_high = self.reg[reg_adr as usize] >> 2;
                let number_low = nibble2 & 0b11;
                let number_high = nibble2 >> 2;

                self.alu.add = true;

                self.alu.reg[0] = reg_num_low;
                self.alu.reg[1] = number_low;
                self.alu.cycles();

                self.reg[7] = self.alu.reg[0]; //temp for the low bytes of the result

                if self.alu.reg[3] == 1 {
                    self.alu.adc = true;
                }

                self.alu.reg[0] = reg_num_high;
                self.alu.reg[1] = number_high;
                self.alu.cycles();

                self.reg[reg_adr as usize] = self.reg[7] | self.alu.reg[0] << 2;
                self.reg[7] = self.alu.reg[3]; //overwrited to hold the carry

                println!("{:04b}", self.reg[reg_adr as usize]);
                println!("{:04b}", self.reg[7]);
            }
            //ADD rx, ry ---> rx
            0b0011 => {
                let reg_adr1 = nibble1;
                let reg_adr2 = nibble2;
                let reg1_num_low = self.reg[reg_adr1 as usize] & 0b11;
                let reg1_num_high = self.reg[reg_adr1 as usize] >> 2;
                let reg2_num_low = self.reg[reg_adr2 as usize] & 0b11;
                let reg2_num_high = self.reg[reg_adr2 as usize] >> 2;

                self.alu.add = true;

                self.alu.reg[0] = reg1_num_low;
                self.alu.reg[1] = reg2_num_low;
                self.alu.cycles();

                self.reg[7] = self.alu.reg[0]; //temp for the low bytes of the result

                if self.alu.reg[3] == 1 {
                    self.alu.adc = true;
                }

                self.alu.reg[0] = reg1_num_high;
                self.alu.reg[1] = reg2_num_high;
                self.alu.cycles();

                self.reg[reg_adr1 as usize] = self.reg[7] | self.alu.reg[0] << 2;
                self.reg[7] = self.alu.reg[3]; //overwrited to hold the carry

                println!("{:04b}", self.reg[nibble1 as usize]);
                println!("{:04b}", self.reg[nibble2 as usize]);

                println!("{:04b}", self.reg[reg_adr1 as usize]);
                println!("{:04b}", self.reg[7]);
            }
            _ => {
                println!("nada")
            }
        }
    }
}
