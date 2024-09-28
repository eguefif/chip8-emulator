use crate::chip8::CPU;

use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

impl CPU {
    pub fn handle_sound(self: &mut CPU) {
        if self.sound_timer > 0 {
            // _stream must live as long as the sink
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            // Add a dummy source of the sake of the example.
            let source = SineWave::new(440.0)
                .take_duration(Duration::from_secs_f32(0.016))
                .amplify(0.20);
            sink.append(source);

            // The sound plays in a separate thread. This call will block the current thread until the sink
            // has finished playing all its queued sounds.
            sink.sleep_until_end();
        }
    }
}
