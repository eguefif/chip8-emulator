#![allow(dead_code)]
#[derive(Debug)]
pub struct Opcode {
    pub opcode: u16,
    pub code: u8,
    pub x: usize,
    pub y: usize,
    pub d: u8,
    pub kk: u8,
    pub nnn: usize,
}

impl Opcode {
    pub fn new(byte1: u8, byte2: u8) -> Opcode {
        let opcode = make_opcode(byte1, byte2);
        let code = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let d = (opcode & 0x000F) as u8;

        let nnn = (opcode & 0x0FFF) as usize;
        let kk: u8 = (opcode & 0x00FF) as u8;

        Opcode {
            opcode,
            code,
            x,
            y,
            d,
            kk,
            nnn,
        }
    }

    pub fn display(self: &Opcode) {
        println!(
            "self: {:x?}, code: {:x?}, x: {:x?}, y:{:x?}, d:{:x?}, kk:{:x?}, nnn:{:x?}",
            self.opcode, self.code, self.x, self.y, self.d, self.kk, self.nnn
        );
    }
}

fn make_opcode(byte1: u8, byte2: u8) -> u16 {
    let opcode1 = byte1 as u16;
    let opcode2 = byte2 as u16;

    opcode1 << 8 | opcode2
}
