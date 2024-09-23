use crate::chip8::CPU;
use crate::config::*;

impl CPU {
    pub fn draw_sprite(self: &mut CPU, vx: u8, vy: u8, d: u8) {
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
}
