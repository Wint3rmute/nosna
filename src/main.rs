use rodio::source::Source;
use rodio::{OutputStream, Sink};
use std::io;
use std::sync::{Arc, RwLock};

use std::time::Duration;

mod adsr;
mod configuration;
mod operator;
mod voice;

use configuration::SynthConfiguration;
use voice::Voice;

static SAMPLE_RATE: usize = 44100;

type SharedSynthConfiguration = Arc<RwLock<SynthConfiguration>>;
type SharedVoice = Arc<RwLock<Voice>>;

struct Synth {
    voice: SharedVoice,
    configuration: SharedSynthConfiguration,
}

impl Iterator for Synth {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0.0;

        let synth_configuration = &self.configuration.read().unwrap();
        let operators_configuration = &synth_configuration.operators_configuration;

        result += self.voice.write().unwrap().tick(operators_configuration);

        Some(result)
    }
}

impl Source for Synth {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let configuration = Arc::new(RwLock::new(SynthConfiguration::new()));
    // let operator = Arc::new(RwLock::new(Operator::new()));
    let voice = Arc::new(RwLock::new(Voice::new()));
    // let voice = Arc::new()
    let source = Synth {
        voice: voice.clone(),
        configuration: configuration,
    };
    // // Add a dummy source of the sake of the example.
    sink.append(source);

    loop {
        let mut input = String::new();
        println!("Enter number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
        if let Ok(num) = input.trim().parse::<f32>() {
            voice.write().unwrap().note_on(num);
            // operator.write().unwrap().base_frequency = num;
            // configuration.operators_configuration[0].base_frequency = num;
            // configuration. // set_frequency(num);
            // source.operators[0].adsr.set_attack(num);
            if num == 0.0 {
                break;
            }
        } else {
            println!("Invalid number");
        }
    }

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
