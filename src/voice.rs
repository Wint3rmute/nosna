use crate::constants;
use crate::{configuration::OperatorConfiguration, operator::Operator};
use rodio::decoder::Decoder;
use std::sync::{Arc, Mutex};

pub struct VoiceState {
    pub base_frequency: f32,
    pub phase_increment: f32,
    pub key_velocity: f32,
    pub note_on: bool,
}

pub struct Voice {
    pub voice_state: VoiceState,
    pub operators: Vec<Operator>,
    pub sample: Sample,
}

// asdadas

pub struct Sample {
    data: Vec<f32>,
    play_position: f32,
    playback_speed: f32,
}

impl Sample {
    fn get_at_index(&self, sample_position: f32) -> f32 {
        let left_sample = sample_position.floor();
        let right_sample = left_sample + 1.0;

        let distance_from_left_sample = sample_position - left_sample;
        let distance_from_right_sample = 1.0 - distance_from_left_sample;

        (self.data[left_sample as usize] as f32 * (distance_from_left_sample))
            + (self.data[right_sample as usize] as f32 * distance_from_right_sample)
        // self.data[left_sample as usize]
    }
}

// dsadasdasd

impl Voice {
    pub fn new() -> Self {
        // let file = std::fs::File::open("./samples/bass_5th.wav").unwrap();
        // let file = std::fs::File::open("./samples/elements_samples_hit_04.wav").unwrap();
        // let file = std::fs::File::open("./samples/small_arpeggio.wav").unwrap();
        let file = std::fs::File::open("./samples/filter_mod_chord.wav").unwrap();
        let mut d = Decoder::new_wav(file).unwrap();
        let mut sample_data: Vec<f32> = vec![];
        while let Some(s) = d.next() {
            sample_data.push(s as f32 / i16::MAX as f32);
            d.next(); // Skip the 2nd channel
        }

        let sample = Sample {
            play_position: 0.0,
            playback_speed: 0.02,
            data: sample_data,
        };

        let mut v = Self {
            voice_state: VoiceState {
                base_frequency: 440.0,
                phase_increment: 0.0,
                key_velocity: 0.0,
                note_on: false,
            },
            operators: vec![Operator::new(), Operator::new(), Operator::new()],
            sample,
        };
        v.set_frequency(440.0);

        v
    }

    pub fn note_on(&mut self, frequency: f32, velocity: f32) {
        self.voice_state.key_velocity = velocity;
        self.voice_state.note_on = true;
        for operator in self.operators.iter_mut() {
            operator.reset();
        }
        // self.sample.play_position = 0.0;
        self.set_frequency(frequency);
    }

    pub fn note_off(&mut self) {
        self.voice_state.note_on = false;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.voice_state.base_frequency = frequency;
        self.voice_state.phase_increment =
            (2.0 * std::f32::consts::PI * frequency) / constants::SAMPLE_RATE as f32;
    }

    pub fn tick(&mut self, configurations: &[OperatorConfiguration]) -> f32 {
        let wave_length = configurations[2].frequency_multiplier;

        let adsr_state = self.operators[1]
            .adsr
            .tick(&configurations[1], self.voice_state.note_on);
        self.sample.play_position += (self.voice_state.base_frequency * wave_length) / 440.0;

        let offset =
            (configurations[1].strength * 10.0 + configurations[1].frequency_multiplier) * 100.0;

        if self.sample.play_position
            >= (constants::SAMPLE_RATE as f32 * wave_length / 440.0) + offset
        {
            self.sample.play_position = 0.0 + offset;
        }
        self.sample.get_at_index(self.sample.play_position) * adsr_state * 2.0
        // let modulation = self.operators[1].tick(0.0, &configurations[1], &self.voice_state)
        //     + self.operators[2].tick(0.0, &configurations[2], &self.voice_state);

        // self.operators[0].tick(modulation, &configurations[0], &self.voice_state)
    }
}
