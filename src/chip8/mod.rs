#![allow(clippy::all)]

use std::io;

pub const VIDEO_MEMORY_SIZE: usize = 64 * 32;
pub const VM_LINE_SIZE: u8 = 64;

#[derive(Debug)]
pub struct CPU {
    pub registers: [u8; 16],
    pub index: usize,
    pub pc: usize,
    pub stack: [u16; 16],
    pub sp: u8,
    pub memory: [u8; 4096],
    pub video_memory: [u8; VIDEO_MEMORY_SIZE],
}

impl CPU {
    pub fn new(rom: Vec<u8>) -> CPU {
        let mut cpu = CPU {
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
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

    pub fn display_debug(self: &mut CPU) {
        print!("Debug mode: ");
        let mut buffer = String::new();
        let count = io::stdin().read_line(&mut buffer).expect("Error input");
        while count > 0 {
            match buffer.as_str() {
                "c" => return,
                "registers" => println!("{:?}", self.registers),
                "memory" => println!("{:?}", self.memory),
                "pc" => println!("{:?}", self.pc),
                "video" => println!("{:?}", self.video_memory),
                "index" => println!("{:?}", self.index),
                "opcode" => println!("{:x?}", self.memory[self.pc]),
                "stack" => println!("{:?}", self.stack),
                "sp" => println!("{:?}", self.sp),
                _ => return,
            }
        }
    }

    fn read_opcode(self: &mut CPU) -> u16 {
        let opcode1 = self.memory[self.pc] as u16;
        let opcode2 = self.memory[self.pc + 1] as u16;

        self.pc += 2;
        opcode1 << 8 | opcode2
    }

    fn display_opcode(self: &mut CPU, opcode: u16) {
        let code = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let kk: u8 = (opcode & 0x00FF) as u8;
        println!(
            "opcode: {:x?}, code: {:x?}, x: {:x?}, y:{:x?}, d:{:x?}, kk:{:x?}, nnn:{:x?}",
            opcode, code, x, y, d, kk, nnn
        );
    }

    pub fn run(self: &mut CPU) -> u8 {
        let opcode: u16 = self.read_opcode();

        let code = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        let nnn = opcode & 0x0FFF;
        let kk: u8 = (opcode & 0x00FF) as u8;
        self.display_opcode(opcode);

        match (code, x, y, d) {
            (0x0, 0x0, 0xE, 0x0) => self.video_memory = [0; VIDEO_MEMORY_SIZE],
            (0x1, _, _, _) => self.pc = nnn as usize,
            (0x6, _, _, _) => self.registers[x as usize] = kk,
            (0x7, _, _, _) => self.add_value(x as usize, kk),
            (0xA, _, _, _) => self.index = nnn as usize,
            (0x0, 0x0, 0x0, 0x0) => return 0,
            (0xD, _, _, _) => self.draw_sprite(x, y, d),
            _ => eprintln!("Opcode not implement yet: {:?}.", opcode),
        }
        return 1;
    }

    fn draw_sprite(self: &mut CPU, vx: u8, vy: u8, d: u8) {
        self.registers[0xF] = 0;
        let x = self.registers[vx as usize] as usize;
        let y = self.registers[vy as usize] as usize;
        let mut sprite: Vec<u8> = vec![0; d as usize];
        self.load_sprite(&mut sprite, d as usize);

        println!("{:?}", sprite);
        println!("x {:?}, y {:?}, d: {:x?}", x, y, d);
        let mut position: usize = y * VM_LINE_SIZE as usize + x;
        for byte in sprite {
            if position >= VIDEO_MEMORY_SIZE {
                break;
            }
            for byte_x in 0..8 {
                if x + byte_x > VM_LINE_SIZE as usize {
                    break;
                }
                let p = position + 8 - byte_x as usize;
                let value = self.video_memory[p];
                let xor_value = (byte >> byte_x) & 0x1;
                self.video_memory[p] = self.custom_xor(value, xor_value);
            }
            position += VM_LINE_SIZE as usize;
        }
    }

    fn custom_xor(self: &mut CPU, value: u8, pixel: u8) -> u8 {
        if (value >= 1 && pixel == 0) || (value == 0 && pixel >= 1) {
            return 0x1;
        }
        0
    }

    fn load_sprite(self: &mut CPU, sprite: &mut Vec<u8>, d: usize) {
        let end: usize = self.index + d;
        let mut i = 0;

        for position in self.index..end {
            sprite[i] = self.memory[position];
            i += 1;
        }
    }

    fn add_value(self: &mut CPU, x: usize, value: u8) {
        let (val, overflow) = self.registers[x].overflowing_add(value);

        self.registers[x] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    pub fn display_video(self: &mut CPU) {
        for pos in 0..VIDEO_MEMORY_SIZE as usize {
            if pos % VM_LINE_SIZE as usize == 0 {
                println!("");
            } else {
                print!("{:?}", self.video_memory[pos]);
            }
        }
        println!("");
    }
}
