#![allow(clippy::all)]

#[derive(Debug)]
pub struct CPU {
    pub registers: [u8; 16],
    pub index: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub memory: [u8; 4096],
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> CPU {
        let mut cpu = CPU {
            registers: [0; 16],
            index: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            memory: [0; 4096],
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
}
