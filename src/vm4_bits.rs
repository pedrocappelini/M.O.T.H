use crate::vm2_bits::Vm2Bits;

pub enum VmState {
    Fetch,
    AddWaitLow {
        dest_reg: u8,
        reg_high: u8,
        val_high: u8,
    },
    AddWaitHigh {
        dest_reg: u8,
    },
    SubWaitLow {
        dest_reg: u8,
        reg_high: u8,
        val_high: u8,
    },
    SubWaitHigh {
        dest_reg: u8,
    },
    Halted,
}

pub struct Vm4Bits {
    pub memory: [u8; 256], //0x11111111
    pub reg: [u8; 8],
    pub stack: [u8; 8],
    pub pc: u8,
    pub index: u8,
    pub sp: u8,
    pub alu: Vm2Bits,

    pub state: VmState,
    pub temp_ops_low: u8, // temp for holding the low bits of the result (in ADD and SUB ops)
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
            state: VmState::Fetch,
            temp_ops_low: 0,
        }
    }

    pub fn cycle(&mut self) {
        match self.state {
            VmState::Fetch => {
                print!("f");
                let instruction = self.memory[self.pc as usize];

                match instruction {
                    0b0000 => {
                        //Halt
                        self.state = VmState::Halted
                    }
                    0b0100 => {
                        //Assign
                        Self::pc_adder(self); // + 1
                        let reg_adr = self.memory[self.pc as usize];
                        Self::pc_adder(self); // + 2
                        let val = self.memory[self.pc as usize];

                        self.reg[reg_adr as usize] = val;

                        Self::pc_adder(self); // + 3, ready for next instruction
                    }
                    0b0001 => {
                        //add immediate (ADD r$, x)
                        Self::pc_adder(self); // + 1
                        let reg_adr = self.memory[self.pc as usize];
                        Self::pc_adder(self); // + 2
                        let val = self.memory[self.pc as usize];

                        let reg_low = self.reg[reg_adr as usize] & 0b11;
                        let reg_high = self.reg[reg_adr as usize] >> 2;
                        let val_low = val & 0b11;
                        let val_high = val >> 2;

                        //setting for the ALU cycle
                        self.alu.add = true;
                        self.alu.adc = false; // reset carry
                        self.alu.reg[0] = reg_low;
                        self.alu.reg[1] = val_low;

                        self.state = VmState::AddWaitLow {
                            dest_reg: reg_adr,
                            reg_high,
                            val_high,
                        };

                        Self::pc_adder(self); // + 3
                    }
                    _ => {}
                }
            }
            VmState::AddWaitLow {
                dest_reg,
                reg_high,
                val_high,
            } => {
                print!("wl");
                self.alu.cycles();
                self.temp_ops_low = self.alu.reg[0];

                self.alu.adc = self.alu.reg[3] == 1;
                self.alu.reg[0] = reg_high;
                self.alu.reg[1] = val_high;

                self.state = VmState::AddWaitHigh { dest_reg }
            }
            VmState::SubWaitLow {
                dest_reg,
                reg_high,
                val_high,
            } => {
                //subwaitlow
            }
            VmState::AddWaitHigh { dest_reg } => {
                //addwaithigh
                self.alu.cycles();
                self.reg[dest_reg as usize] = self.temp_ops_low | (self.alu.reg[0] << 2);

                self.reg[7] = self.alu.reg[3];

                println!("{:04b}", self.reg[dest_reg as usize]);

                self.state = VmState::Fetch;
            }
            VmState::SubWaitHigh { dest_reg } => {
                //subwaithigh
            }
            VmState::Halted => {}
        }
        // let instruction = self.memory[self.pc as usize];
        // Self::pc_adder(self);
        // let nibble1 = self.memory[self.pc as usize];
        // Self::pc_adder(self);
        // let nibble2 = self.memory[self.pc as usize];
        // Self::pc_adder(self);

        // match instruction {
        //     //halt
        //     0b0000 => {}
        //     //assigns a value to a register
        //     0b0100 => {
        //         let reg_adr = nibble1;
        //         self.reg[reg_adr as usize] = nibble2;
        //     }
        //     //ADD r$, x
        //     0b0001 => {
        //         let reg_adr = nibble1;
        //         let reg_num_low = self.reg[reg_adr as usize] & 0b11;
        //         let reg_num_high = self.reg[reg_adr as usize] >> 2;
        //         let number_low = nibble2 & 0b11;
        //         let number_high = nibble2 >> 2;

        //         self.alu.add = true;

        //         self.alu.reg[0] = reg_num_low;
        //         self.alu.reg[1] = number_low;
        //         self.alu.cycles();

        //         self.reg[7] = self.alu.reg[0]; //temp for the low bytes of the result

        //         if self.alu.reg[3] == 1 {
        //             self.alu.adc = true;
        //         }

        //         self.alu.reg[0] = reg_num_high;
        //         self.alu.reg[1] = number_high;
        //         self.alu.cycles();

        //         self.reg[reg_adr as usize] = self.reg[7] | self.alu.reg[0] << 2;
        //         self.reg[7] = self.alu.reg[3]; //overwrited to hold the carry

        //         println!("{:04b}", self.reg[reg_adr as usize]);
        //         println!("{:04b}", self.reg[7]);
        //     }
        //     //ADD rx, ry ---> rx
        //     0b0011 => {
        //         let reg_adr1 = nibble1;
        //         let reg_adr2 = nibble2;
        //         let reg1_num_low = self.reg[reg_adr1 as usize] & 0b11;
        //         let reg1_num_high = self.reg[reg_adr1 as usize] >> 2;
        //         let reg2_num_low = self.reg[reg_adr2 as usize] & 0b11;
        //         let reg2_num_high = self.reg[reg_adr2 as usize] >> 2;

        //         println!("{:04b}", self.reg[nibble1 as usize]);
        //         println!("{:04b}", self.reg[nibble2 as usize]);

        //         self.alu.add = true;

        //         self.alu.reg[0] = reg1_num_low;
        //         self.alu.reg[1] = reg2_num_low;
        //         self.alu.cycles();

        //         self.reg[7] = self.alu.reg[0]; //temp for the low bytes of the result

        //         if self.alu.reg[3] == 1 {
        //             self.alu.adc = true;
        //         }

        //         self.alu.reg[0] = reg1_num_high;
        //         self.alu.reg[1] = reg2_num_high;
        //         self.alu.cycles();

        //         self.reg[reg_adr1 as usize] = self.reg[7] | self.alu.reg[0] << 2;
        //         self.reg[7] = self.alu.reg[3]; //overwrited to hold the carry

        //         println!("{:04b}", self.reg[reg_adr1 as usize]);
        //         println!("{:04b}", self.reg[7]);
        //     }
        //     //SUB rx, ry -----> rx
        //     0b0010 => {
        //         let reg_adr1 = nibble1;
        //         let reg_adr2 = nibble2;
        //         let reg1_num_low = self.reg[reg_adr1 as usize] & 0b11;
        //         let reg1_num_high = self.reg[reg_adr1 as usize] >> 2;
        //         let reg2_num_low = self.reg[reg_adr2 as usize] & 0b11;
        //         let reg2_num_high = self.reg[reg_adr2 as usize] >> 2;

        //         println!("{:04b}", self.reg[nibble1 as usize]);
        //         println!("{:04b}", self.reg[nibble2 as usize]);

        //         self.alu.add = false;

        //         self.alu.reg[0] = reg1_num_low;
        //         self.alu.reg[1] = reg2_num_low;
        //         self.alu.cycles();

        //         self.reg[7] = self.alu.reg[0];

        //         if self.alu.reg[3] == 1 {
        //             self.alu.sbb = true;
        //         }

        //         self.alu.reg[0] = reg1_num_high;
        //         self.alu.reg[1] = reg2_num_high;
        //         self.alu.cycles();

        //         self.reg[reg_adr1 as usize] = self.reg[7] | self.alu.reg[0] << 2;
        //         self.reg[7] = self.alu.reg[3]; //Overwrites the borrow

        //         println!("{:04b}", self.reg[reg_adr1 as usize]);
        //         println!("{:04b}", self.reg[7]);
        //     }
        //     _ => {
        //         println!("nada")
        //     }
        // }
    }

    pub fn pc_adder(&mut self) {
        let mut b = 1;
        while b != 0 {
            let carry = self.pc & b;
            self.pc = self.pc ^ b;
            b = carry << 1;
        }
    }
}
