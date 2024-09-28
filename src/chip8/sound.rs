use std::time::Duration;

use rodio::{source::SineWave, Sink, Source};

use crate::chip8::CPU;

impl CPU {
    pub fn start_sound(self: &mut CPU, stream_handle: rodio::OutputStreamHandle) {
        self.sink = Sink::try_new(&stream_handle).unwrap();
    }
    pub fn handle_sound(self: &mut CPU) {
        if self.sound_timer > 0 {
            let duration = Duration::from_millis(18);
            let source = SineWave::new(440.0).take_duration(duration);
            self.sink.append(source);
            self.sound_timer -= 1;
        }
    }
}
