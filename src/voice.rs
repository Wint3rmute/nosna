pub struct Voice {
    pub sample_rate: usize,
    pub base_frequency: f32,
    pub phase_increment: f32,
    pub key_velocity: Option<f32>,
}

impl Voice {
    pub fn new() -> Self {
        let mut v = Self {
            sample_rate: 44100,
            base_frequency: 440.0,
            phase_increment: 0.0,
            key_velocity: Some(1.0),
        };
        v.set_frequency(440.0);

        v
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.base_frequency = frequency;
        self.phase_increment = (2.0 * std::f32::consts::PI * frequency) / self.sample_rate as f32;
    }
}
