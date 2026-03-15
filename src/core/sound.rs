use rodio::{
    MixerDeviceSink, Player,
    source::{SineWave, Source},
};
use std::time::Duration;

pub struct Sound {
    _stream: MixerDeviceSink,
    sink: Player,
}

impl Sound {
    pub fn new() -> Self {
        let stream = rodio::DeviceSinkBuilder::open_default_sink().unwrap();
        let sink = Player::connect_new(stream.mixer());
        Sound {
            _stream: stream,
            sink,
        }
    }

    pub fn beep(&self) {
        if self.sink.empty() {
            let source = SineWave::new(440.0)
                .take_duration(Duration::from_millis(100))
                .amplify(0.2);
            self.sink.append(source);
        }
    }
}
