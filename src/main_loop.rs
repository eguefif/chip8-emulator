use crate::chip8::CPU;
use crate::graphics::draw_screen;

use piston_window::*;

pub fn main_loop(window: &mut PistonWindow, cpu: &mut CPU) {
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            cpu.update_keyboard(key);
        }
        if cpu.run() == 0 {
            break;
        }
        draw_screen(window, event, cpu.video_memory);
    }
}
