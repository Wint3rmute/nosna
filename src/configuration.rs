pub struct SynthConfiguration {
    pub sample_rate: usize,
    pub operators_configuration: Vec<OperatorConfiguration>,
    pub voice_states: Vec<Voice>,
}

impl SynthConfiguration {
    pub fn new() -> Self {
        SynthConfiguration {
            sample_rate: 44100,
            operators_configuration: vec![OperatorConfiguration::new()],
            voice_states: vec![Voice::new()],
        }
    }
}

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

pub struct OperatorConfiguration {
    pub sample_rate: usize,

    pub attack: f32,
    pub decay: f32,
    pub sustain: f32,
    pub release: f32,
    pub strength: f32,

    pub frequency_multiplier: f32,
    pub velocity_sensitivity: f32,
}

impl OperatorConfiguration {
    pub fn new() -> Self {
        let mut configuration = OperatorConfiguration {
            sample_rate: 44100,

            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,

            strength: 0.0,
            frequency_multiplier: 1.0,
            velocity_sensitivity: 1.0,
        };

        configuration.set_attack(0.01);
        configuration.set_decay(0.5);

        configuration
    }

    fn set_attack(&mut self, attack: f32) {
        self.attack = 1.0 / (attack * self.sample_rate as f32);
    }

    fn set_decay(&mut self, decay: f32) {
        self.decay = 1.0 / (decay * self.sample_rate as f32);
    }

    fn set_sustain(&mut self, sustain: f32) {
        self.sustain = sustain; //= 1.0 / (sustain * self.sample_rate as f32);
    }

    fn set_release(&mut self, release: f32) {
        self.release = 1.0 / (release * self.sample_rate as f32);
    }
}
