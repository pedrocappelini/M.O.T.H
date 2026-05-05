use vm2_bits::Vm2Bits;

mod vm2_bits;

fn main() {
    let mut summer = Vm2Bits::new();

    summer.reg[0b00] = 1; // first operand
    summer.reg[0b01] = 2; // second operand
    summer.reg[0b10] = 0;
    summer.reg[0b11] = 0; // carry

    summer.add = false;

    dbg!(summer.cycles());
}
