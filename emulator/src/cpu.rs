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

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(rom);
    }

    pub fn fetch(&mut self) -> u16 {
        let opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[(self.pc + 1) as usize] as u16;
        // indexing an array needs usize so thats why I made pc into usize and yeah I forgot about
        // it at first at least I wont forget about it now

        self.pc = self.pc + 2;

        opcode
    }

    pub fn execute(&mut self, opcode: u16) {
        let nibbles = ((opcode & 0xF000) >> 12, (opcode & 0x0F00) >> 8, (opcode & 0x00F0) >> 4, (opcode & 0x000F));

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.display = [[false; 64]; 32],
            (0x1, _, _, _) => self.pc = nnn,
            (0x6, _, _, _) => self.registers[x] = nn,
            (0x7, _, _, _) => self.registers[x] = self.registers[x].wrapping_add(nn),
            (0xA, _, _, _) => self.index = nnn,
            (0xD, _, _, _) => self.draw(x, y, n),
            _ => println!("{:#06X}", opcode),
        };
    }

    pub fn draw(&mut self, x: usize, y: usize, n: usize) {
        let vx = self.registers[x] as usize;
        let vy = self.registers[y] as usize;

        self.registers[0xF] = 0;

        for row in 0..n {
            let sprite_byte = self.memory[self.index as usize + row];
            for col in 0..8 {
                if sprite_byte & (0x80 >> col) != 0 {
                    let px = (vx + col) % 64;
                    let py = (vy + row) % 32;

                    if self.display[py][px] {
                        self.registers[0xF] = 1;
                    }
                    self.display[py][px] ^= true;
                }
            }
        }
    }

    pub fn render(&mut self) {
        for row in &self.display {
            for &pixel in row {
                print!("{}", if pixel { '█' } else { ' ' });
            }
            println!();
        }
    }
}
