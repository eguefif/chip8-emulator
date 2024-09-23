use crate::graphics::{draw_screen, get_windows};
use crate::init::get_cpu;

mod chip8;
mod config;
mod graphics;
mod init;

fn main() {
    let mut window = get_windows();
    let mut cpu = get_cpu();
    while let Some(event) = window.next() {
        if cpu.run() == 0 {
            break;
        }
        let image = cpu.video_memory;
        draw_screen(&mut window, event, image);
    }
}
