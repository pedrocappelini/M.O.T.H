pub struct Vm2Bits {
    pub reg: [u8; 4],
    pub add: bool,
    pub adc: bool,
}

impl Vm2Bits {
    pub fn new() -> Self {
        Self {
            reg: [0; 4],
            add: true,
            adc: false,
        }
    }

    pub fn cycles(&mut self) {
        let mut a = self.reg[0];
        let mut b = self.reg[1];

        let mut previous_carry = self.reg[3];

        if self.add {
            while b != 0 {
                let carry = a & b;
                a = a ^ b;
                b = carry << 1;
            }
        } else {
            while b != 0 {
                let borrow = (!a) & b;
                a = a ^ b;
                b = borrow << 1;
            }
        }

        if self.adc {
            while previous_carry != 0 {
                let carry = a & previous_carry;
                a = a ^ previous_carry;
                previous_carry = carry << 1;
            }
            self.adc = false;
            self.reg[3] = 0;
        }

        self.reg[0] = a & 0b11;
        self.reg[3] = (a >> 2) & 0b01;

        self.reg[1] = 0;
    }
}
