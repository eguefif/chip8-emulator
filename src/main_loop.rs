use std::thread;
use std::time::{Duration, Instant};

use crate::chip8::CPU;
use crate::graphics::draw_screen;

use piston_window::*;

static CLOCK_DURATION: Duration = Duration::from_millis(16);

pub fn main_loop(window: &mut PistonWindow, cpu: &mut CPU) {
    while let Some(event) = window.next() {
        let now = Instant::now();
        println!("Time now: {:?}", now);
        if let Some(Button::Keyboard(key)) = event.press_args() {
            cpu.update_keyboard(key);
        }
        if cpu.run() == 0 {
            break;
        }
        draw_screen(window, event, cpu.video_memory);
        while now.elapsed() <= CLOCK_DURATION {
            let ten_millis = Duration::from_millis(1);
            thread::sleep(ten_millis);
        }
    }
}
