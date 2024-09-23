use crate::graphics::get_windows;
use crate::init::get_cpu;
use crate::main_loop::main_loop;

mod chip8;
mod config;
mod graphics;
mod init;
mod main_loop;

fn main() {
    let mut window = get_windows();
    let mut cpu = get_cpu();
    main_loop(&mut window, &mut cpu);
}
