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

    #[allow(unused_variables)]
    pub fn skip_if_key_pressed(self: &mut CPU, vx: usize) {
        eprintln!("not implemented");
        self.pc += 2;
    }

    #[allow(unused_variables)]
    pub fn skip_if_key_not_pressed(self: &mut CPU, vx: usize) {
        eprintln!("not implemented");
        self.pc += 2;
    }

    #[allow(unused_variables)]
    pub fn wait_for_key(self: &mut CPU) {
        eprintln!("not implemented");
    }

    pub fn set_delay_timer(self: &mut CPU, vx: usize) {
        self.delay_timer = self.registers[vx];
    }

    pub fn set_sound_timer(self: &mut CPU, vx: usize) {
        self.sound_timer = self.registers[vx];
    }
}
