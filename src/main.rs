use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_file: String,
}

fn read_file(f: &mut File) -> std::vec::Vec<u16> {
    let mut rom: Vec<u16> = vec![];
    loop {
        let result = f.read_u16::<LittleEndian>();
        let instruction = match result {
            Ok(byte) => byte,
            Err(error) => {
                eprintln!("Error while reading rom: {error:?}");
                break;
            }
        };
        rom.push(instruction);
    }
    rom
}

fn main() {
    let args = Args::parse();
    let rom_filename = args.rom_file;

    let mut f = File::open(&rom_filename).expect("Unable to open file.");
    let rom = read_file(&mut f);
    for instruction in rom {
        println!("{:0x}", instruction);
    }
}
