use std::time::{Duration, Instant};

use crate::chip8::CPU;
use crate::graphics::draw_screen;

use piston_window::*;

static CLOCK_DURATION: Duration = Duration::from_millis(16);

pub fn main_loop(window: &mut PistonWindow, cpu: &mut CPU) {
    while let Some(event) = window.next() {
        let now = Instant::now();
        handle_keyboard(cpu, &event);
        if cpu.run() == 0 {
            break;
        }
        draw_screen(window, event, cpu.video_memory);
        while now.elapsed() <= CLOCK_DURATION {}
    }
}

fn handle_keyboard(cpu: &mut CPU, event: &Event) {
    if let Some(Button::Keyboard(key)) = event.press_args() {
        cpu.update_keyboard(key, true);
    }
    if let Some(Button::Keyboard(key)) = event.release_args() {
        cpu.update_keyboard(key, false);
    }
}
