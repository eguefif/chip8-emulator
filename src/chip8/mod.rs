#![allow(clippy::all)]

use debug::display_debug;
use piston_window::Key;

use crate::chip8::font::write_fonts_in_memory;
use crate::chip8::keyboard::Keyboard;
use crate::chip8::opcode::Opcode;
use crate::config::*;

mod conditional;
mod debug;
mod draw_sprite;
mod font;
mod keyboard;
mod op_math;
mod opcode;
mod sound;
mod system;

#[derive(Debug)]
pub struct CPU {
    pub registers: [u8; 16],
    pub index: usize,
    pub pc: usize,
    pub stack: [usize; 16],
    pub sp: usize,
    pub memory: [u8; 4096],
    pub video_memory: [u8; VIDEO_MEMORY_SIZE],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub keyboard: Keyboard,
    pub debug: bool,
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
            delay_timer: 0,
            sound_timer: 0,
            keyboard: Keyboard::new(),
            debug: false,
        };
        let mut position = 0x200;

        write_fonts_in_memory(&mut cpu.memory);
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
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
            return 1;
        }
        self.handle_sound();
        self.pc += 2;

        if self.debug {
            opcode.display();
            display_debug(self);
        }

        match (opcode.code, opcode.x, opcode.y, opcode.d) {
            (0x0, 0x0, 0x0, 0x0) => return 0,
            (0x0, 0x0, 0xE, 0x0) => self.video_memory = [0; VIDEO_MEMORY_SIZE],
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.pc = opcode.nnn,
            (0x2, _, _, _) => self.call(opcode.nnn),
            (0x3, _, _, _) => self.skip_if_equal(opcode.x, opcode.kk),
            (0x4, _, _, _) => self.skip_if_different(opcode.x, opcode.kk),
            (0x5, _, _, _) => self.skip_if_vx_vy_equal(opcode.x, opcode.y),
            (0x6, _, _, _) => self.registers[opcode.x] = opcode.kk,
            (0x7, _, _, _) => self.add_value_to_x(opcode.x, opcode.kk),
            (0x8, _, _, 0x0) => self.registers[opcode.x] = self.registers[opcode.y],
            (0x8, _, _, 0x1) => self.registers[opcode.x] |= self.registers[opcode.y],
            (0x8, _, _, 0x2) => self.registers[opcode.x] &= self.registers[opcode.y],
            (0x8, _, _, 0x3) => self.registers[opcode.x] ^= self.registers[opcode.y],
            (0x8, _, _, 0x4) => self.add_value_to_x(opcode.x, self.registers[opcode.y]),
            (0x8, _, _, 0x5) => self.sub_value_from_x(opcode.x, self.registers[opcode.y]),
            (0x8, _, _, 0x6) => self.shr_vx(opcode.x),
            (0x8, _, _, 0x7) => self.sub_from_vy(opcode.x, opcode.y),
            (0x8, _, _, 0xE) => self.shl_vx(opcode.x),
            (0x9, _, _, 0) => self.skip_if_vx_vy_different(opcode.x, opcode.y),
            (0xA, _, _, _) => self.index = opcode.nnn,
            (0xB, _, _, _) => self.pc = opcode.nnn + self.registers[0] as usize,
            (0xC, _, _, _) => self.registers[opcode.x] = rand::random::<u8>(),
            (0xD, _, _, _) => self.draw_sprite(opcode.x, opcode.y, opcode.d),
            (0xE, _, 0x9, 0xE) => self.skip_if_key_pressed(opcode.x),
            (0xE, _, 0xA, 0x1) => self.skip_if_key_not_pressed(opcode.x),
            (0xF, _, 0x0, 0xA) => self.wait_for_key(opcode.x),
            (0xF, _, 0x1, 0x5) => self.set_delay_timer(opcode.x),
            (0xF, _, 0x1, 0x8) => self.set_sound_timer(opcode.x),
            (0xF, _, 0x1, 0xE) => self.index += self.registers[opcode.x] as usize,
            (0xF, _, 0x2, 0x9) => self.set_font(opcode.x),
            (0xF, _, 0x3, 0x3) => self.set_bcd(opcode.x),
            (0xF, _, 0x5, 0x5) => self.save_registers(opcode.x),
            (0xF, _, 0x6, 0x5) => self.read_registers(opcode.x),
            _ => eprintln!("Opcode not implement yet: {:?}.", opcode),
        }
        return 1;
    }

    pub fn update_keyboard(self: &mut CPU, key: Key) {
        self.keyboard.toggle_key(key);
    }

    pub fn toggle_debug(self: &mut CPU) {
        self.debug = !self.debug;
    }
}
