use crate::font::FONT_SET;

const FONT_START: usize = 0x050;

pub struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: usize,
    delay: u8,
    sound: u8,
    display: [[bool; 64]; 32],
    keypad: [bool; 16],
}

impl Chip8 {
    #[rustfmt::skip] // putting this here so formatter doesnt colapse the fn to a couple of lines
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay: 0,
            sound: 0,
            display: [[false; 64]; 32],
            keypad: [false; 16],
        };

        chip8.memory[FONT_START..FONT_START + FONT_SET.len()].copy_from_slice(&FONT_SET);

        debug_assert_eq!(chip8.memory[FONT_START], 0xf0);

        chip8
    }
}
