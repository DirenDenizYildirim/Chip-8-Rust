pub struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    index: u16,
    pc: u16,
    // stack goes here
    delay: u8, // they might be in the integer type but I aint sure
    sound: u8,
    display: [[bool; 64]; 32],
    keypad: [bool; 16],
}
