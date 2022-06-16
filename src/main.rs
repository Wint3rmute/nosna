use midly::{live::LiveEvent, MidiMessage};
use rodio::source::Source;
use rodio::{OutputStream, Sink};
use std::io;
use std::sync::{Arc, RwLock};

use std::time::Duration;

mod adsr;
mod configuration;
mod midi_input;
mod operator;
#[cfg(feature = "gui")]
mod oscilloscope;
mod voice;

use configuration::SynthConfiguration;
use voice::Voice;

static SAMPLE_RATE: usize = 44100;

type SharedSynthConfiguration = Arc<RwLock<SynthConfiguration>>;
type SharedVoiceManager = Arc<RwLock<VoiceManager>>;

pub struct VoiceManager {
    voices: Vec<Voice>,
    voice_index: usize,
}

impl VoiceManager {
    fn new() -> Self {
        Self {
            voices: std::iter::repeat_with(Voice::new).take(4).collect(),
            voice_index: 0,
        }
    }

    fn note_on(&mut self, note: f32) {
        self.voices[self.voice_index].note_on(note);
        self.voice_index += 1;

        if self.voice_index >= self.voices.len() {
            self.voice_index = 0;
        }
    }

    fn tick(&mut self, configuration: &SynthConfiguration) -> f32 {
        self.voices
            .iter_mut()
            .map(|voice| voice.tick(configuration.operators_configuration.as_slice()))
            .sum()
    }
}

struct Synth {
    voice_manager: SharedVoiceManager,
    configuration: SharedSynthConfiguration,
    samples: Samples,
    sample_index: usize,
}

type Samples = Arc<RwLock<Vec<f32>>>;
impl Iterator for Synth {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = 0.0;

        let synth_configuration = &self.configuration.read().unwrap();

        result += self
            .voice_manager
            .write()
            .unwrap()
            .tick(synth_configuration);

        let mut samples = self.samples.write().unwrap();
        samples[self.sample_index] = result;
        self.sample_index += 1;
        if self.sample_index >= samples.len() {
            self.sample_index = 0;
        }

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
    let samples: Samples = Arc::new(RwLock::new(vec![0.0_f32; 1024]));

    let configuration = Arc::new(RwLock::new(SynthConfiguration::new()));
    let voice_manager = Arc::new(RwLock::new(VoiceManager::new()));
    let source = Synth {
        voice_manager: voice_manager.clone(),
        configuration: configuration.clone(),
        samples: samples.clone(),
        sample_index: 0,
    };
    sink.append(source);

    let (in_port, midi_in) = midi_input::midi_test().unwrap();

    let vm = voice_manager.clone();
    let _conn_in = midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());

            let event = LiveEvent::parse(message).unwrap();
            match event {
                LiveEvent::Midi { channel, message } => match message {
                    MidiMessage::NoteOn { key, vel } => {
                        println!("hit note {}, {} on channel {}", key, vel, channel);
                    }
                    _ => {}
                },
                _ => {
                    println!("No idea what this is")
                }
            }

            if message[2] == 0 {
                return;
            }

            let frequency = 440.0 * (2.0_f32).powf((message[1] as f32 - 69.0) as f32 / 12.0);
            println!("{}", frequency);
            voice_manager.write().unwrap().note_on(frequency);
        },
        // callback,
        (),
    );

    let voice_manager = vm;

    #[cfg(feature = "gui")]
    oscilloscope::run_synth_ui(voice_manager.clone(), samples.clone());

    loop {
        let mut input = String::new();
        println!("Enter number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Not a valid string");
        if let Ok(num) = input.trim().parse::<f32>() {
            voice_manager.write().unwrap().note_on(num);
            // voice.write().unwrap().note_on(num);
            // operator.write().unwrap().base_frequency = num;
            // configuration.operators_configuration[0].base_frequency = num;
            // configuration. // set_frequency(num);
            // source.operators[0].adsr.set_attack(num);
            if num < 0.0 {
                // voice_manager.write().unwrap().no
            }
        } else {
            println!("Invalid number");
            break;
        }
    }

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
