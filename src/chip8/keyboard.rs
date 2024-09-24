use crate::chip8::CPU;

impl CPU {
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
}
