pub struct SynthConfiguration {
    pub sample_rate: usize,
    pub operators_configuration: Vec<OperatorConfiguration>,
}

impl SynthConfiguration {
    pub fn new() -> Self {
        SynthConfiguration {
            sample_rate: 44100,
            operators_configuration: vec![
                OperatorConfiguration::new(1.0),
                OperatorConfiguration::new(1.0),
            ],
        }
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
    pub fn new(frequency_multiplier: f32) -> Self {
        let mut configuration = OperatorConfiguration {
            sample_rate: 44100,

            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,

            strength: 1.0,
            velocity_sensitivity: 1.0,
            frequency_multiplier,
        };

        configuration.set_attack(0.01);
        configuration.set_decay(0.2);
        configuration.set_sustain(0.00);
        configuration.set_release(1.0);

        configuration
    }

    pub fn set_attack(&mut self, attack: f32) {
        self.attack = 1.0 / (attack * self.sample_rate as f32);
    }

    pub fn set_decay(&mut self, decay: f32) {
        self.decay = 1.0 / (decay * self.sample_rate as f32);
    }

    pub fn set_sustain(&mut self, sustain: f32) {
        self.sustain = sustain; //= 1.0 / (sustain * self.sample_rate as f32);
    }

    pub fn set_release(&mut self, release: f32) {
        self.release = 1.0 / (release * self.sample_rate as f32);
    }
}
