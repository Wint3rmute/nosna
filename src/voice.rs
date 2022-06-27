use crate::{configuration::OperatorConfiguration, operator::Operator};
use rand::prelude::*;
use std::iter;

pub struct Voice {
    pub sample_rate: usize,
    pub base_frequency: f32,
    pub phase_increment: f32,
    pub key_velocity: f32,
    pub note_on: bool,
    pub operators: Vec<Operator>,
    pub wavetable: Vec<f32>,
    pub wavetable_index: usize,
    pub wavetable_size: usize,
}

impl Voice {
    pub fn new() -> Self {
        let mut v = Self {
            sample_rate: 44100,
            base_frequency: 440.0,
            phase_increment: 0.0,
            key_velocity: 0.0,
            note_on: false,
            operators: vec![Operator::new()],
            wavetable: iter::repeat(0.0).take(2048).collect(),
            wavetable_size: 100,
            wavetable_index: 0,
        };
        v.set_frequency(440.0);

        v
    }

    pub fn note_on(&mut self, frequency: f32, velocity: f32) {
        self.set_frequency(frequency);
        self.wavetable_size = (100000.0 / frequency) as usize;
        self.key_velocity = velocity;
        self.note_on = true;
        for operator in self.operators.iter_mut() {
            operator.reset();
        }

        let mut rng = rand::thread_rng();
        for sample in self.wavetable.iter_mut() {
            *sample = (rng.gen::<f32>() - 0.5) * velocity * 2.0; //* frequency / 440.0; // / 2.0;
                                                                 // *sample = y;
                                                                 // sample = rng.gen();
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
        let previous_value = self.wavetable[self.wavetable_index];

        self.wavetable_index += 1;
        if self.wavetable_index >= self.wavetable_size {
            self.wavetable_index = 0;
        }

        let a = configurations[0].attack;
        let one_minus_a = 1.0 - a;
        let result = self.wavetable[self.wavetable_index];
        self.wavetable[self.wavetable_index] =
            (self.wavetable[self.wavetable_index] * a + previous_value * one_minus_a); // / 2.0;

        result
        // let mut result = 0.0;
        // for (operator, configuration) in self.operators.iter_mut().zip(configurations) {
        //     result += operator.tick(
        //         0.0,
        //         configuration,
        //         self.phase_increment,
        //         self.key_velocity,
        //         self.note_on,
        //     );
        // }

        // result
    }
}
