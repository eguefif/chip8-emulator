use std::collections::HashMap;

use piston_window::Key;

use crate::chip8::CPU;

impl CPU {
    #[allow(unused_variables)]
    pub fn skip_if_key_pressed(self: &mut CPU, vx: usize) {
        if self.keyboard.is_key_pressed(self.registers[vx]) {
            self.pc += 2;
        }
    }

    #[allow(unused_variables)]
    pub fn skip_if_key_not_pressed(self: &mut CPU, vx: usize) {
        if !self.keyboard.is_key_pressed(self.registers[vx]) {
            self.pc += 2;
        }
    }

    #[allow(unused_variables)]
    pub fn wait_for_key(self: &mut CPU, vx: usize) {
        if self.keyboard.is_key_pressed(self.registers[vx]) {
            self.pc -= 2;
        }
    }
}

#[derive(Debug)]
pub struct Keyboard {
    pub keys: HashMap<String, bool>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: HashMap::from([
                (String::from("0"), false),
                (String::from("1"), false),
                (String::from("2"), false),
                (String::from("3"), false),
                (String::from("4"), false),
                (String::from("5"), false),
                (String::from("6"), false),
                (String::from("7"), false),
                (String::from("8"), false),
                (String::from("9"), false),
                (String::from("A"), false),
                (String::from("B"), false),
                (String::from("C"), false),
                (String::from("D"), false),
                (String::from("E"), false),
                (String::from("F"), false),
            ]),
        }
    }

    pub fn toggle_key(self: &mut Keyboard, key: Key) {
        match key {
            Key::NumPad1 => self
                .keys
                .entry(String::from("1"))
                .and_modify(|value| *value = !(*value)),
            Key::NumPad2 => self
                .keys
                .entry(String::from("2"))
                .and_modify(|value| *value = !(*value)),
            Key::NumPad3 => self
                .keys
                .entry(String::from("3"))
                .and_modify(|value| *value = !(*value)),
            Key::NumPad4 => self
                .keys
                .entry(String::from("C"))
                .and_modify(|value| *value = !(*value)),
            Key::Q => self
                .keys
                .entry(String::from("4"))
                .and_modify(|value| *value = !(*value)),
            Key::W => self
                .keys
                .entry(String::from("5"))
                .and_modify(|value| *value = !(*value)),
            Key::E => self
                .keys
                .entry(String::from("6"))
                .and_modify(|value| *value = !(*value)),
            Key::R => self
                .keys
                .entry(String::from("D"))
                .and_modify(|value| *value = !(*value)),
            Key::A => self
                .keys
                .entry(String::from("7"))
                .and_modify(|value| *value = !(*value)),
            Key::S => self
                .keys
                .entry(String::from("8"))
                .and_modify(|value| *value = !(*value)),
            Key::D => self
                .keys
                .entry(String::from("9"))
                .and_modify(|value| *value = !(*value)),
            Key::F => self
                .keys
                .entry(String::from("E"))
                .and_modify(|value| *value = !(*value)),
            Key::Z => self
                .keys
                .entry(String::from("A"))
                .and_modify(|value| *value = !(*value)),
            Key::X => self
                .keys
                .entry(String::from("0"))
                .and_modify(|value| *value = !(*value)),
            Key::C => self
                .keys
                .entry(String::from("B"))
                .and_modify(|value| *value = !(*value)),
            Key::V => self
                .keys
                .entry(String::from("F"))
                .and_modify(|value| *value = !(*value)),
            _ => return,
        };
    }

    pub fn is_key_pressed(self: &mut Keyboard, key: u8) -> bool {
        match key {
            0 => *self.keys.get(&String::from("0")).unwrap(),
            1 => *self.keys.get(&String::from("1")).unwrap(),
            2 => *self.keys.get(&String::from("2")).unwrap(),
            3 => *self.keys.get(&String::from("3")).unwrap(),
            4 => *self.keys.get(&String::from("4")).unwrap(),
            5 => *self.keys.get(&String::from("5")).unwrap(),
            6 => *self.keys.get(&String::from("6")).unwrap(),
            7 => *self.keys.get(&String::from("7")).unwrap(),
            8 => *self.keys.get(&String::from("8")).unwrap(),
            9 => *self.keys.get(&String::from("9")).unwrap(),
            10 => *self.keys.get(&String::from("A")).unwrap(),
            11 => *self.keys.get(&String::from("B")).unwrap(),
            12 => *self.keys.get(&String::from("C")).unwrap(),
            13 => *self.keys.get(&String::from("D")).unwrap(),
            14 => *self.keys.get(&String::from("E")).unwrap(),
            15 => *self.keys.get(&String::from("F")).unwrap(),
            _ => false,
        }
    }
}
