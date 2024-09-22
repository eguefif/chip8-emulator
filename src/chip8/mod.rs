#![allow(clippy::all)]

const VIDEO_MEMORY_SIZE: usize = 64 * 32;

#[derive(Debug)]
pub struct CPU {
    pub registers: [u8; 16],
    pub index: u16,
    pub pc: usize,
    pub stack: [u16; 16],
    pub sp: u8,
    pub memory: [u8; 4096],
    pub video_memory: [u32; VIDEO_MEMORY_SIZE],
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> CPU {
        let mut cpu = CPU {
            registers: [0; 16],
            index: 0,
            pc: 200,
            stack: [0; 16],
            sp: 0,
            memory: [0; 4096],
            video_memory: [0; VIDEO_MEMORY_SIZE],
        };
        let mut position = 200;

        for byte in rom {
            cpu.memory[position] = byte;
            position += 1;
            if position > cpu.memory.len() {
                break;
            }
        }

        cpu
    }

    pub fn run(self: &mut CPU) {
        let mut opcode1 = self.memory[self.pc] as u16;
        let opcode2 = self.memory[self.pc + 1] as u16;

        opcode1 = opcode1 << 8;
        let opcode: u16 = opcode1 + opcode2;

        let code = (opcode & 0xF000) >> 12;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let d = opcode & 0x000F;

        let nnn = opcode & 0x0FFF;
        let kk: u8 = (opcode & 0x00FF) as u8;

        match (code, x, y, d) {
            (0x0, 0x0, 0xE, 0x0) => self.video_memory = [0; VIDEO_MEMORY_SIZE],
            (0x1, _, _, _) => self.pc = nnn as usize,
            (0x6, _, _, _) => self.registers[x as usize] = kk,
            (0x7, _, _, _) => self.add_value(x as usize, kk),
            (0xA, _, _, _) => self.index = nnn,
            (0xD, _, _, _) => self.draw_pixel(x, y, d),
            _ => eprintln!("Opcode not implement yet: {:?}.", opcode),
        }
    }

    fn draw_pixel(self: &mut CPU, x: u16, y: u16, d: u16) {}

    fn add_value(self: &mut CPU, x: usize, value: u8) {
        let (val, overflow) = self.registers[x].overflowing_add(value);

        self.registers[x] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
