use crate::cpu::Chip8;
use std::fs;

mod cpu;
mod font;

fn main() {
    let mut chip = Chip8::new();

    let rom = fs::read("roms/2-ibm-logo.ch8").expect("couldnt read rom");
    println!("the rom is {}", rom.len());
    chip.load_rom(&rom);

    for _ in 0..30 {
        let opcode = chip.fetch();
        chip.execute(opcode);
    }
    chip.render();
}
