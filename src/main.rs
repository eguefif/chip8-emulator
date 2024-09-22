use crate::chip8::CPU;
use clap::Parser;
use piston_window::*;
use rectangle::square;
use std::fs::File;
use std::io::prelude::*;

pub mod chip8;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_file: String,
}

fn read_file(f: &mut File) -> std::vec::Vec<u8> {
    let mut rom: Vec<u8> = vec![];
    let mut buffer = [0; 1];

    while f.read_exact(&mut buffer).is_ok() {
        rom.push(buffer[0]);
    }
    rom
}

fn main() {
    let args = Args::parse();
    let rom_filename = args.rom_file;

    let mut f = File::open(rom_filename).expect("Unable to open file.");
    let rom = read_file(&mut f);
    let mut cpu = CPU::new(rom);
    let mut window: PistonWindow = WindowSettings::new("Chip8-emulator", [640, 320])
        .exit_on_esc(true)
        .build()
        .expect("Cannot open a window");
    while let Some(event) = window.next() {
        if cpu.run() == 0 {
            break;
        }

        let image = cpu.video_memory;
        window.draw_2d(&event, |ctx, renderer, _device| {
            clear([1.0, 1.0, 1.0, 1.0], renderer);
            for (i, pixel) in image.iter().enumerate() {
                let x = (i % chip8::VM_LINE_SIZE as usize) as f64 * 10.0;
                let y = (i / chip8::VM_LINE_SIZE as usize) as f64 * 10.0;
                if *pixel > 0 {
                    let rectangle = Rectangle::new(color::BLACK);
                    let dims = square(x, y, 10.0);
                    rectangle.draw(dims, &ctx.draw_state, ctx.transform, renderer);
                }
            }
        });
    }
}
