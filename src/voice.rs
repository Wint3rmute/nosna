use crate::{configuration::OperatorConfiguration, operator::Operator};

pub struct Voice {
    pub sample_rate: usize,
    pub base_frequency: f32,
    pub phase_increment: f32,
    pub key_velocity: Option<f32>,
    pub operators: Vec<Operator>,
}

impl Voice {
    pub fn new() -> Self {
        let mut v = Self {
            sample_rate: 44100,
            base_frequency: 440.0,
            phase_increment: 0.0,
            key_velocity: None,
            operators: vec![Operator::new(), Operator::new()],
        };
        v.set_frequency(440.0);

        v
    }

    pub fn note_on(&mut self, frequency: f32) {
        self.set_frequency(frequency);
        self.key_velocity = Some(1.0);
        for operator in self.operators.iter_mut() {
            operator.reset();
        }
    }

    pub fn note_off(&mut self) {
        self.key_velocity = None;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.base_frequency = frequency;
        self.phase_increment = (2.0 * std::f32::consts::PI * frequency) / self.sample_rate as f32;
    }

    pub fn tick(&mut self, configurations: &[OperatorConfiguration]) -> f32 {
        let mut result = 0.0;
        for (operator, configuration) in self.operators.iter_mut().zip(configurations) {
            result += operator.tick(0.0, configuration, self.phase_increment, self.key_velocity);
        }

        result
    }
}
