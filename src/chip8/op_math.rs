use crate::chip8::CPU;

impl CPU {
    pub fn add_value(self: &mut CPU, x: usize, value: u8) {
        let (val, overflow) = self.registers[x].overflowing_add(value);

        self.registers[x] = val;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
