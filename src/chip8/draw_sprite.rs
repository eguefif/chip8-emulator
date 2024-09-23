use crate::chip8::CPU;
use crate::config::*;

impl CPU {
    pub fn draw_sprite(self: &mut CPU, vx: u8, vy: u8, d: u8) {
        self.reset_collision_flag();
        let (x, y) = self.get_coordinate(vx, vy);
        let sprite = self.load_sprite(d as usize);
        self.draw_sprite_in_video_memory(sprite, x, y);
    }

    fn reset_collision_flag(self: &mut CPU) {
        self.registers[0xF] = 0;
    }

    fn get_coordinate(self: &mut CPU, vx: u8, vy: u8) -> (usize, usize) {
        let x = self.registers[vx as usize] as usize;
        let y = self.registers[vy as usize] as usize;
        (x, y)
    }

    fn load_sprite(self: &mut CPU, d: usize) -> Vec<u8> {
        let end: usize = self.index + d;
        let mut sprite: Vec<u8> = vec![0; d as usize];
        let mut i = 0;

        for position in self.index..end {
            sprite[i] = self.memory[position];
            i += 1;
        }
        sprite
    }

    fn draw_sprite_in_video_memory(self: &mut CPU, sprite: Vec<u8>, x: usize, y: usize) {
        let mut position: usize = y * VM_LINE_SIZE as usize + x;
        for byte in sprite {
            if self.is_out_of_memory(position) {
                break;
            }
            self.draw_byte(byte, position, x);
            position += VM_LINE_SIZE as usize;
        }
    }

    fn is_out_of_memory(self: &mut CPU, position: usize) -> bool {
        position >= VIDEO_MEMORY_SIZE
    }

    fn draw_byte(self: &mut CPU, byte: u8, position: usize, x: usize) {
        for byte_x in 0..8 {
            if self.is_out_of_line(x + byte_x) {
                break;
            }
            let p = position + 8 - byte_x as usize;
            let value = self.video_memory[p];
            let xor_value = (byte >> byte_x) & 0x1;
            self.video_memory[p] = value ^ xor_value;
            self.check_collision(value, self.video_memory[p]);
        }
    }

    fn is_out_of_line(self: &mut CPU, x: usize) -> bool {
        x > (VM_LINE_SIZE as usize)
    }

    fn check_collision(self: &mut CPU, value: u8, new_value: u8) {
        if value >= 1 && new_value == 0 {
            self.registers[0xF] = 1;
        }
    }
}
