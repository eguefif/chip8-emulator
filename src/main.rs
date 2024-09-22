use crate::chip8::CPU;
use clap::Parser;
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

    let mut f = File::open(&rom_filename).expect("Unable to open file.");
    let rom = read_file(&mut f);
    let cpu = CPU::new(rom);
    println!("{:?}", cpu);
}
