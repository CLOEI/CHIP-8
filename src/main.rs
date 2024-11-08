use std::{fs::File, io::Read};

use register::Register;

mod disassembler;
mod register;

fn main() {
    let mut rom = File::open("Pong.ch8").expect("Failed to open file");
    let mut register = Register::default();

    disassembler::start(&mut register, &mut rom);
}
