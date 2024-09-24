use crate::chip8::CPU;

impl CPU {
    pub fn add_value_to_x(self: &mut CPU, x: usize, value: u8) {
        let (val, overflow) = self.registers[x].overflowing_add(value);

        self.registers[x] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    pub fn sub_value_from_x(self: &mut CPU, vx: usize, value: u8) {
        let x = self.registers[vx];
        if x > value {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        (self.registers[vx], _) = x.overflowing_sub(value);
    }

    pub fn shr_vx(self: &mut CPU, vx: usize) {
        self.registers[0xF] = self.registers[vx] & 0x1;
        (self.registers[vx], _) = self.registers[vx].overflowing_shr(1);
    }

    pub fn shl_vx(self: &mut CPU, vx: usize) {
        self.registers[0xF] = (self.registers[vx] >> 7) & 0x1;
        (self.registers[vx], _) = self.registers[vx].overflowing_shl(1);
    }

    pub fn sub_from_vy(self: &mut CPU, vx: usize, vy: usize) {
        let x = self.registers[vx];
        let y = self.registers[vy];
        self.registers[0xF] = 0;
        if y > x {
            self.registers[0xF] = 1;
        }
        self.registers[vx] = y - x;
    }

    pub fn set_bcd(self: &mut CPU, vx: usize) {
        let mut value: u8 = self.registers[vx];

        self.memory[self.index + 2] = value % 10;
        value /= 10;

        self.memory[self.index + 1] = value % 10;
        value /= 10;

        self.memory[self.index] = value % 10;
    }
}
