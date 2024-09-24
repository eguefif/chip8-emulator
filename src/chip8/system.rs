use crate::chip8::CPU;

const FONTSET_START_ADDRESS: usize = 0x50;

impl CPU {
    pub fn save_registers(self: &mut CPU, vx: usize) {
        let last = self.registers[vx] as usize;

        for x in 0..=last {
            self.memory[self.index + x] = self.registers[x];
        }
    }

    pub fn read_registers(self: &mut CPU, vx: usize) {
        let last = self.registers[vx] as usize;

        for x in 0..=last {
            self.registers[x] = self.memory[self.index + x];
        }
    }

    pub fn call(self: &mut CPU, nnn: usize) {
        if self.sp >= 16 {
            panic!("Stack overflow");
        }
        self.stack[self.sp] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    pub fn ret(self: &mut CPU) {
        if self.sp == 0 {
            panic!("Stack underflow");
        }
        self.sp -= 1;
        self.pc = self.stack[self.sp];
    }

    pub fn set_delay_timer(self: &mut CPU, vx: usize) {
        self.delay_timer = self.registers[vx];
    }

    pub fn set_sound_timer(self: &mut CPU, vx: usize) {
        self.sound_timer = self.registers[vx];
    }

    pub fn set_font(self: &mut CPU, vx: usize) {
        let position: usize = self.registers[vx] as usize;
        self.index = FONTSET_START_ADDRESS + (5 * position);
    }
}
