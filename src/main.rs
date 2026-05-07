use vm4_bits::Vm4Bits;

mod vm2_bits;
mod vm4_bits;

fn main() {
    let mut vm4bits = Vm4Bits::new();

    vm4bits.memory[0] = 0b0100;
    vm4bits.memory[1] = 0b0010;
    vm4bits.memory[2] = 0b0001;

    vm4bits.memory[3] = 0b0001;
    vm4bits.memory[4] = 0b0010;
    vm4bits.memory[5] = 0b0010;

    // vm4bits.memory[3] = 0b0100;
    // vm4bits.memory[4] = 0b0011;
    // vm4bits.memory[5] = 0b0011;

    // vm4bits.memory[6] = 0b0011;
    // vm4bits.memory[7] = 0b0010;
    // vm4bits.memory[8] = 0b0011;

    loop {
        vm4bits.cycle();
        if vm4bits.pc > 200 {
            break;
        }
    }
}
