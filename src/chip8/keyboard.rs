use piston_window::Key;

use crate::chip8::CPU;

impl CPU {
    pub fn skip_if_key_pressed(self: &mut CPU, vx: usize) {
        if self.keyboard.is_key_pressed(self.registers[vx]) {
            self.pc += 2;
        }
    }

    pub fn skip_if_key_not_pressed(self: &mut CPU, vx: usize) {
        if !self.keyboard.is_key_pressed(self.registers[vx]) {
            self.pc += 2;
        }
    }

    pub fn wait_for_key(self: &mut CPU, vx: usize) {
        let mut key = 0;

        if !self.keyboard.is_a_key_pressed(&mut key) {
            self.pc -= 2;
        } else {
            self.registers[vx] = key as u8;
        }
    }
}

#[derive(Debug)]
pub struct Keyboard {
    pub keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn set_key(self: &mut Keyboard, key: Key, value: bool) {
        let index = get_index(key);
        self.keys[index] = value;
    }

    pub fn is_key_pressed(self: &mut Keyboard, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn is_a_key_pressed(self: &mut Keyboard, reg: &mut usize) -> bool {
        for (i, key) in self.keys.iter().enumerate() {
            if *key {
                *reg = i;
                return true;
            }
        }
        return false;
    }
}

fn get_index(key: Key) -> usize {
    match key {
        Key::D1 => 1,
        Key::D2 => 2,
        Key::D3 => 3,
        Key::D4 => 0xc,
        Key::Q => 4,
        Key::W => 5,
        Key::E => 6,
        Key::R => 0xd,
        Key::A => 7,
        Key::S => 8,
        Key::D => 9,
        Key::F => 0xe,
        Key::Z => 0xa,
        Key::X => 0,
        Key::C => 0xb,
        Key::V => 0xf,
        _ => 0,
    }
}
