#![allow(clippy::all)]

use crate::chip8::opcode::Opcode;
use crate::config::*;

mod debug;
mod draw_sprite;
mod op_math;
mod opcode;

#[derive(Debug)]
pub struct CPU {
    pub registers: [u8; 16],
    pub index: usize,
    pub pc: usize,
    //pub stack: [u16; 16],
    //pub sp: u8,
    pub memory: [u8; 4096],
    pub video_memory: [u8; VIDEO_MEMORY_SIZE],
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> CPU {
        let mut cpu = CPU {
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            //stack: [0; 16],
            //sp: 0,
            memory: [0; 4096],
            video_memory: [0; VIDEO_MEMORY_SIZE],
        };
        let mut position = 0x200;

        for byte in rom {
            cpu.memory[position] = byte;
            position += 1;
            if position > cpu.memory.len() {
                break;
            }
        }

        cpu
    }

    pub fn run(self: &mut CPU) -> u8 {
        let opcode: Opcode = Opcode::new(self.memory[self.pc], self.memory[self.pc + 1]);
        self.pc += 2;

        opcode.display();

        match (opcode.code, opcode.x, opcode.y, opcode.d) {
            (0x0, 0x0, 0xE, 0x0) => self.video_memory = [0; VIDEO_MEMORY_SIZE],
            (0x1, _, _, _) => self.pc = opcode.nnn as usize,
            (0x6, _, _, _) => self.registers[opcode.x as usize] = opcode.kk,
            (0x7, _, _, _) => self.add_value(opcode.x as usize, opcode.kk),
            (0xA, _, _, _) => self.index = opcode.nnn as usize,
            (0x0, 0x0, 0x0, 0x0) => return 0,
            (0xD, _, _, _) => self.draw_sprite(opcode.x, opcode.y, opcode.d),
            _ => eprintln!("Opcode not implement yet: {:?}.", opcode),
        }
        return 1;
    }
}
