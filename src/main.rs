use vm2_bits::Vm2Bits;
use vm4_bits::Vm4Bits;

mod vm2_bits;
mod vm4_bits;

fn main() {
    let mut vm4bits = Vm4Bits::new();

    vm4bits.memory[0] = 0b0001;
    vm4bits.memory[3] = 0b0000;
    loop {
        vm4bits.cycle();
    }
}
