use crate::chip8::CPU;
use crate::config::*;

impl CPU {
    pub fn draw_sprite(self: &mut CPU, vx: usize, vy: usize, d: u8) {
        self.reset_collision_flag();
        let (x, y) = self.get_coordinate(vx, vy);
        let sprite = self.load_sprite(d as usize);
        self.draw_sprite_in_video_memory(sprite, x, y);
    }

    fn reset_collision_flag(self: &mut CPU) {
        self.registers[0xF] = 0;
    }

    fn get_coordinate(self: &mut CPU, vx: usize, vy: usize) -> (usize, usize) {
        let x = self.registers[vx] as usize;
        let y = self.registers[vy] as usize;
        (x, y)
    }

    fn load_sprite(self: &mut CPU, d: usize) -> Vec<u8> {
        self.memory[self.index..(self.index + d)].to_vec()
    }

    fn draw_sprite_in_video_memory(self: &mut CPU, sprite: Vec<u8>, x: usize, y: usize) {
        let mut position: usize =
            (y % VM_COL_SIZE as usize) * VM_LINE_SIZE as usize + (x % VM_LINE_SIZE as usize);
        for byte in sprite {
            if self.is_out_of_memory(position) {
                break;
            }
            self.draw_byte(byte, position, x % VM_LINE_SIZE as usize);
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
            let p = position + byte_x;
            let value = self.video_memory[p];
            let xor_value = (byte >> (7 - byte_x)) & 0x1;
            self.video_memory[p] = value ^ xor_value;
            self.check_collision(value, xor_value);
        }
    }

    fn is_out_of_line(self: &mut CPU, x: usize) -> bool {
        x > (VM_LINE_SIZE as usize)
    }

    fn check_collision(self: &mut CPU, value: u8, byte_value: u8) {
        if value >= 1 && byte_value >= 1 {
            self.registers[0xF] = 1;
        }
    }
}
