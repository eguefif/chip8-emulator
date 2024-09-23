#![allow(dead_code)]
use std::io;

use crate::chip8::CPU;

pub fn display_debug(cpu: &mut CPU) {
    print!("Debug mode: ");
    let mut buffer = String::new();
    let count = io::stdin().read_line(&mut buffer).expect("Error input");
    while count > 0 {
        match buffer.as_str() {
            "c" => return,
            "registers" => println!("{:?}", cpu.registers),
            "memory" => println!("{:?}", cpu.memory),
            "pc" => println!("{:?}", cpu.pc),
            "video" => println!("{:?}", cpu.video_memory),
            "index" => println!("{:?}", cpu.index),
            "opcode" => println!("{:x?}", cpu.memory[cpu.pc]),
            "stack" => println!("{:?}", cpu.stack),
            "sp" => println!("{:?}", cpu.sp),
            _ => return,
        }
    }
}
