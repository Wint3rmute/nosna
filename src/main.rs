use rodio::source::Source;
use rodio::{OutputStream, Sink};
use std::io;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use std::time::Duration;

// mod adsr;
mod adsr;
mod configuration;
mod operator;

// use adsr::ADSR;
use configuration::{OperatorConfiguration, SynthConfiguration, Voice};
use operator::Operator;

static SAMPLE_RATE: usize = 44100;

type SharedSynthConfiguration = Arc<RwLock<SynthConfiguration>>;
// type VoiceStates = Arc<RwLock<Vec<Voice>>>;
type SharedOperator = Arc<RwLock<Operator>>;

struct Synth {
    operator: SharedOperator,
    configuration: SharedSynthConfiguration,
}

impl Iterator for Synth {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0.0;

        let ref synth_configuration = self.configuration.read().unwrap();
        let ref operators_configuration = synth_configuration.operators_configuration;
        let ref voice_states = synth_configuration.voice_states;

        result +=
            self.operator
                .write()
                .unwrap()
                .tick(0.0, &operators_configuration[0], &voice_states[0]);
        // for (sine, configuration) in self.sines.iter().zip(sine_configurations.iter()) {
        // configuration.
        // result += sine.tick(configuration)
        // }

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
    let operator = Arc::new(RwLock::new(Operator::new()));
    let source = Synth {
        operator: operator.clone(),
        configuration: configuration.clone(),
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
            let ref mut configuration = configuration.write().unwrap(); //.operators[0].adsr.reset();
                                                                        // configuration.sine_configurations[0].set_frequency(num);
            operator.write().unwrap().reset();
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
