pub struct Vm2Bits {
    pub reg: [u8; 4],
    pub add: bool,
}

impl Vm2Bits {
    pub fn new() -> Self {
        Self {
            reg: [0; 4],
            add: true,
        }
    }

    pub fn cycles(&mut self) {
        let mut a = self.reg[0];
        let mut b = self.reg[1];

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

        self.reg[0] = a & 0b11;
        dbg!(&self.reg[0]);
        self.reg[3] = (a >> 2) & 0b01;
        dbg!(&self.reg[3]);

        self.reg[1] = 0;
    }
}
