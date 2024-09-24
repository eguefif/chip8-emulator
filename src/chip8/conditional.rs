use crate::chip8::CPU;

impl CPU {
    pub fn skip_if_equal(self: &mut CPU, vx: usize, kk: u8) {
        let value = self.registers[vx];
        if value == kk {
            self.pc += 2;
        }
    }

    pub fn skip_if_different(self: &mut CPU, vx: usize, kk: u8) {
        let value = self.registers[vx];
        if value != kk {
            self.pc += 2;
        }
    }

    pub fn skip_if_vx_vy_equal(self: &mut CPU, vx: usize, vy: usize) {
        let x = self.registers[vx];
        let y = self.registers[vy];
        if x == y {
            self.pc += 2;
        }
    }

    pub fn skip_if_vx_vy_different(self: &mut CPU, vx: usize, vy: usize) {
        let x = self.registers[vx];
        let y = self.registers[vy];
        if x != y {
            self.pc += 2;
        }
    }
}
