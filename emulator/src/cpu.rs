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
