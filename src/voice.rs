use crate::{configuration::OperatorConfiguration, operator::Operator};

pub struct Voice {
    pub sample_rate: usize,
    pub base_frequency: f32,
    pub phase_increment: f32,
    pub key_velocity: f32,
    pub note_on: bool,
    pub operators: Vec<Operator>,
}

impl Voice {
    pub fn new() -> Self {
        let mut v = Self {
            sample_rate: 44100,
            base_frequency: 440.0,
            phase_increment: 0.0,
            key_velocity: 0.0,
            note_on: false,
            operators: vec![Operator::new(), Operator::new()],
        };
        v.set_frequency(440.0);

        v
    }

    pub fn note_on(&mut self, frequency: f32, velocity: f32) {
        self.set_frequency(frequency);
        self.key_velocity = velocity;
        self.note_on = true;
        for operator in self.operators.iter_mut() {
            operator.reset();
        }
    }

    pub fn note_off(&mut self) {
        self.note_on = false;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.base_frequency = frequency;
        self.phase_increment = (2.0 * std::f32::consts::PI * frequency) / self.sample_rate as f32;
    }

    pub fn tick(&mut self, configurations: &[OperatorConfiguration]) -> f32 {
        let modulation = self.operators[1].tick(
            0.0,
            &configurations[1],
            self.phase_increment,
            self.key_velocity,
            self.note_on,
        );

        self.operators[0].tick(
            modulation,
            &configurations[0],
            self.phase_increment,
            self.key_velocity,
            self.note_on,
        )
    }
}
