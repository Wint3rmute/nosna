use rodio::source::Source;
use rodio::{OutputStream, Sink};
use std::io;
use std::sync::Arc;
use std::sync::Mutex;

use std::time::Duration;

struct Operator {
    phase_increment: f32,
    phase: f32,
    adsr: ADSR,
}

enum Phase {
    ATTACK,
    DECAY,
    SUSTAIN,
    RELEASE,
    FINISHED,
}

struct ADSR {
    sample_rate: usize,
    phase: Phase,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: f32,
}

impl ADSR {
    fn tick(&mut self, note_on: bool) -> f32 {
        if !note_on {
            self.phase = Phase::RELEASE;
            self.state -= self.release;
            if self.state < 0.0 {
                self.state = 0.0
            }

            return self.state;
        }

        match self.phase {
            Phase::ATTACK => {
                self.state += self.attack;
                if self.state > 1.0 {
                    self.state = 1.0;
                    self.phase = Phase::DECAY;
                }
            }
            _ => {}
        }
        self.state
    }

    fn reset(&mut self) {
        self.state = 0.0;
        self.phase = Phase::ATTACK;
    }

    fn set_attack(&mut self, attack: f32) {
        self.attack = 1.0 / (attack * self.sample_rate as f32);
    }

    fn set_decay(&mut self, decay: f32) {
        self.decay = decay * self.sample_rate as f32;
    }

    fn set_sustain(&mut self, sustain: f32) {
        self.sustain = sustain * self.sample_rate as f32;
    }

    fn set_release(&mut self, release: f32) {
        self.release = release * self.sample_rate as f32;
    }

    fn new(sample_rate: usize) -> Self {
        let adsr = ADSR {
            sample_rate,
            phase: Phase::ATTACK,
            attack: 0.0,
            decay: 0.0,
            sustain: 0.0,
            release: 0.0,
            state: 0.0,
        };

        // adsr.set_attack(attack);

        adsr
    }
}

impl Operator {
    fn new(frequency: f32, sample_rate: usize) -> Self {
        let mut adsr = ADSR::new(sample_rate);
        adsr.set_attack(10.0);

        Self {
            phase_increment: std::f32::consts::PI * 2.0 * frequency / sample_rate as f32,
            phase: 0.0,
            adsr,
        }
    }

    fn tick(&mut self, modulation: f32) -> f32 {
        self.phase += self.phase_increment;

        if self.phase > std::f32::consts::PI * 2.0 {
            self.phase -= std::f32::consts::PI * 2.0;
        }

        (self.phase + modulation).sin() * self.adsr.tick(true)
    }
}

struct FmVoice {
    sample_rate: usize,
    operators: Vec<Operator>,
}

impl FmVoice {
    pub fn new(frequency: f32, sample_rate: usize) -> Self {
        FmVoice {
            sample_rate,
            operators: vec![
                Operator::new(frequency * 1.0, sample_rate),
                Operator::new(frequency, sample_rate),
            ],
        }
    }
}

struct SharedFmVoice(Arc<Mutex<FmVoice>>);

impl Iterator for SharedFmVoice {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.lock().unwrap().next()
    }
}

impl Source for SharedFmVoice {
    fn sample_rate(&self) -> u32 {
        self.0.lock().unwrap().sample_rate()
    }

    fn current_frame_len(&self) -> Option<usize> {
        self.0.lock().unwrap().current_frame_len()
    }

    fn channels(&self) -> u16 {
        1
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl Iterator for FmVoice {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let modulation = 0.0; //self.operators[1].tick(0.0);
                              // let modulation = self.operators[1].tick(0.0);
        Some(self.operators[0].tick(modulation * 1.1))
    }
}

impl Source for FmVoice {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.sample_rate)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        44100
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let shared_source = SharedFmVoice {
        0: Arc::new(Mutex::new(FmVoice::new(440.0, 44100))),
    };

    let source = shared_source.0.clone();

    sink.append(shared_source);
    sink.play();

    loop {
        let mut input = String::new();
        println!("Enter number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
        if let Ok(num) = input.trim().parse::<f32>() {
            let mut source = source.lock().unwrap(); //.operators[0].adsr.reset();
            source.operators[0].adsr.reset();
            source.operators[0].adsr.set_attack(num);
            if num == 0.0 {
                break;
            }
        } else {
            println!("Invalid number");
        }
    }
    // source.volume = 1.5;

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
