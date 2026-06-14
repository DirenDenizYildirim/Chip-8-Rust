use crate::cpu::Chip8;
use std::fs;

mod cpu;
mod font;

fn main() {
    let mut chip = Chip8::new();

    let rom = fs::read("roms/2-ibm-logo.ch8").expect("couldnt read rom");
    println!("the rom is {}", rom.len());
    chip.load_rom(&rom);

    let opcode = chip.fetch();
    println!("first opcode: {:#06X}", opcode);
}
