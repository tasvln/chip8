pub struct Chip8 {
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub index: u16,       // index register (I)
    pub pc: u16,          // program counter
    pub stack: [u16; 16], // call stack
    pub sp: u8,           // stack pointer
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub display: [bool; 64 * 32],
    pub keypad: [bool; 16],
}

const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; 64 * 32],
            keypad: [false; 16],
        };

        // load font into memory starting at 0x000
        for (i, byte) in FONT.iter().enumerate() {
            chip8.memory[i] = *byte;
        }

        chip8
    }

    pub fn load_rom(&mut self, path: &str) {
        let data = std::fs::read(path).expect("Failed to read ROM");

        for (i, byte) in data.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }

        println!("Loaded {} bytes into memory", data.len());
    }

    pub fn cycle(&mut self) {
        // read 2 bytes from memory and combine them into one opcode
        let opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);

        // move PC forward by 2 (each instruction is 2 bytes)
        self.pc += 2;

        // DECODE + EXECUTE
        let x = ((opcode & 0x0F00) >> 8) as usize; // second nibble
        let y = ((opcode & 0x00F0) >> 4) as usize; // third nibble
        let n = (opcode & 0x000F) as u8; // fourth nibble
        let nn = (opcode & 0x00FF) as u8; // last byte
        let nnn = opcode & 0x0FFF; // last 3 nibbles

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self.display = [false; 64 * 32], // clear screen
                0x00EE => {
                    // return from subroutine
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => {}
            },
            0x1000 => self.pc = nnn, // jump
            0x2000 => {
                // call subroutine
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            0x3000 => {
                if self.registers[x] == nn {
                    self.pc += 2
                }
            } // skip if VX == NN
            0x4000 => {
                if self.registers[x] != nn {
                    self.pc += 2
                }
            } // skip if VX != NN
            0x5000 => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2
                }
            } // skip if VX == VY
            0x6000 => self.registers[x] = nn, // set VX to NN
            0x7000 => self.registers[x] = self.registers[x].wrapping_add(nn), // add NN to VX
            0x8000 => match opcode & 0x000F {
                0x0 => self.registers[x] = self.registers[y], // set VX to VY
                0x1 => self.registers[x] |= self.registers[y], // VX OR VY
                0x2 => self.registers[x] &= self.registers[y], // VX AND VY
                0x3 => self.registers[x] ^= self.registers[y], // VX XOR VY
                0x4 => {
                    // VX + VY
                    let (result, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                    self.registers[0xF] = if overflow { 1 } else { 0 };
                    self.registers[x] = result;
                }
                0x5 => {
                    // VX - VY
                    let (result, borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                    self.registers[0xF] = if borrow { 0 } else { 1 };
                    self.registers[x] = result;
                }
                0x6 => {
                    // shift right
                    self.registers[0xF] = self.registers[x] & 0x1;
                    self.registers[x] >>= 1;
                }
                0x7 => {
                    // VY - VX
                    let (result, borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                    self.registers[0xF] = if borrow { 0 } else { 1 };
                    self.registers[x] = result;
                }
                0xE => {
                    // shift left
                    self.registers[0xF] = (self.registers[x] & 0x80) >> 7;
                    self.registers[x] <<= 1;
                }
                _ => {}
            },
            0x9000 => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2
                }
            } // skip if VX != VY
            0xA000 => self.index = nnn, // set index register
            0xB000 => self.pc = nnn + self.registers[0] as u16, // jump + V0
            0xC000 => {
                // random
                let random: u8 = rand::random();
                self.registers[x] = random & nn;
            }
            0xD000 => {
                // draw sprite
                let x_pos = self.registers[x] as usize % 64;
                let y_pos = self.registers[y] as usize % 32;
                self.registers[0xF] = 0;

                for row in 0..n {
                    let sprite_byte = self.memory[self.index as usize + row as usize];
                    for col in 0..8 {
                        if sprite_byte & (0x80 >> col) != 0 {
                            let px = (x_pos + col) % 64;
                            let py = (y_pos + row as usize) % 32;
                            let idx = py * 64 + px;
                            if self.display[idx] {
                                self.registers[0xF] = 1; // collision
                            }
                            self.display[idx] ^= true;
                        }
                    }
                }
            }
            0xE000 => match opcode & 0x00FF {
                0x9E => {
                    if self.keypad[self.registers[x] as usize] {
                        self.pc += 2
                    }
                } // skip if key pressed
                0xA1 => {
                    if !self.keypad[self.registers[x] as usize] {
                        self.pc += 2
                    }
                } // skip if key not pressed
                _ => {}
            },
            0xF000 => match opcode & 0x00FF {
                0x07 => self.registers[x] = self.delay_timer, // get delay timer
                0x0A => {
                    // wait for key press
                    let key = self.keypad.iter().position(|&k| k);
                    match key {
                        Some(k) => self.registers[x] = k as u8,
                        None => self.pc -= 2, // rewind PC, keep waiting
                    }
                }
                0x15 => self.delay_timer = self.registers[x], // set delay timer
                0x18 => self.sound_timer = self.registers[x], // set sound timer
                0x1E => self.index += self.registers[x] as u16, // add VX to index
                0x29 => self.index = self.registers[x] as u16 * 5, // font character
                0x33 => {
                    // binary coded decimal
                    self.memory[self.index as usize] = self.registers[x] / 100;
                    self.memory[self.index as usize + 1] = (self.registers[x] / 10) % 10;
                    self.memory[self.index as usize + 2] = self.registers[x] % 10;
                }
                0x55 => {
                    // store registers to memory
                    for i in 0..=x {
                        self.memory[self.index as usize + i] = self.registers[i];
                    }
                }
                0x65 => {
                    // load registers from memory
                    for i in 0..=x {
                        self.registers[i] = self.memory[self.index as usize + i];
                    }
                }
                _ => {}
            },
            _ => println!("Unknown opcode: {:#X}", opcode),
        }
    }
}
