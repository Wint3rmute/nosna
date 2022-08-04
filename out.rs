#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use macroquad::prelude::*;
use midly::{live::LiveEvent, MidiMessage};
use rodio::source::Source;
use rodio::{OutputStream, Sink};
use std::io;
use std::sync::{Arc, RwLock};

use std::time::Duration;

mod adsr {
    // #[cfg(feature = "gui")]



























    // callback,


    // task::bl


    // ui.

    // voice.write().unwrap().note_on(num);
    // operator.write().unwrap().base_frequency = num;
    // configuration.operators_configuration[0].base_frequency = num;
    // configuration. // set_frequency(num);
    // source.operators[0].adsr.set_attack(num);
    // voice_manager.write().unwrap().no

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    use crate::configuration::OperatorConfiguration;
    pub enum Phase { Attack, Decay, Sustain, Release, Finished, }
    pub struct Adsr {
        phase: Phase,
        state: f32,
    }
    impl Adsr {
        pub fn tick(&mut self, configuration: &OperatorConfiguration,
            note_on: bool) -> f32 {
            if note_on {
                    match self.phase {
                        Phase::Attack => {
                            self.state += configuration.attack;
                            if self.state > 1.0 {
                                    self.state = 1.0;
                                    self.phase = Phase::Decay;
                                }
                        }
                        Phase::Decay => {
                            self.state -= configuration.decay;
                            if self.state < configuration.sustain {
                                    self.phase = Phase::Sustain;
                                    self.state = configuration.sustain;
                                }
                        }
                        Phase::Sustain => {}
                        Phase::Release | Phase::Finished => {
                            self.state = 0.0;
                            self.phase = Phase::Attack
                        }
                    }
                } else {
                   match self.phase {
                       Phase::Finished => {}
                       Phase::Release => {
                           self.state -= configuration.release;
                           if self.state < 0.0 {
                                   self.state = 0.0;
                                   self.phase = Phase::Finished;
                               }
                       }
                       _ => { self.phase = Phase::Release; }
                   }
               }
            self.state
        }
        pub fn reset(&mut self) {
            self.state = 0.0;
            self.phase = Phase::Attack;
        }
        pub fn new() -> Self { Self { state: 0.0, phase: Phase::Attack } }
    }
}
mod configuration {
    pub struct SynthConfiguration {
        pub sample_rate: usize,
        pub operators_configuration: Vec<OperatorConfiguration>,
    }
    impl SynthConfiguration {
        pub fn new() -> Self {
            SynthConfiguration {
                sample_rate: 44100,
                operators_configuration: <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([OperatorConfiguration::new(1.0)])),
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
            let mut configuration =
                OperatorConfiguration {
                    sample_rate: 44100,
                    attack: 0.0,
                    decay: 0.0,
                    sustain: 0.0,
                    release: 0.0,
                    strength: 0.0,
                    velocity_sensitivity: 1.0,
                    frequency_multiplier,
                };
            configuration.set_attack(0.01);
            configuration.set_decay(0.5);
            configuration.set_sustain(0.9);
            configuration.set_release(0.1);
            configuration
        }
        fn set_attack(&mut self, attack: f32) {
            self.attack = 1.0 / (attack * self.sample_rate as f32);
        }
        fn set_decay(&mut self, decay: f32) {
            self.decay = 1.0 / (decay * self.sample_rate as f32);
        }
        fn set_sustain(&mut self, sustain: f32) { self.sustain = sustain; }
        fn set_release(&mut self, release: f32) {
            self.release = 1.0 / (release * self.sample_rate as f32);
        }
    }
}
mod midi_input {
    use midir::{Ignore, MidiInput, MidiInputPort};
    use std::error::Error;
    use std::io::{stdin, stdout, Write};
    pub fn midi_test() -> Result<(MidiInputPort, MidiInput), Box<dyn Error>> {
        let input = String::new();
        let mut midi_in = MidiInput::new("midir reading input")?;
        midi_in.ignore(Ignore::None);
        let in_ports = midi_in.ports();
        let in_port =
            match in_ports.len() {
                0 => return Err("no input port found".into()),
                1 => {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Choosing the only available input port: ",
                                            "\n"],
                                &[::core::fmt::ArgumentV1::new_display(&midi_in.port_name(&in_ports[0]).unwrap())]));
                    };
                    &in_ports[0]
                }
                _ => {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["\nAvailable input ports:\n"],
                                &[]));
                    };
                    for (i, p) in in_ports.iter().enumerate() {
                        {
                            ::std::io::_print(::core::fmt::Arguments::new_v1(&["", ": ",
                                                "\n"],
                                    &[::core::fmt::ArgumentV1::new_display(&i),
                                                ::core::fmt::ArgumentV1::new_display(&midi_in.port_name(p).unwrap())]));
                        };
                    }
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["Please select input port: "],
                                &[]));
                    };
                    stdout().flush()?;
                    let mut input = String::new();
                    stdin().read_line(&mut input)?;
                    in_ports.get(input.trim().parse::<usize>()?).ok_or("invalid input port selected")?
                }
            };
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["\nOpening connection\n"],
                    &[]));
        };
        let in_port_name = midi_in.port_name(in_port)?;
        Ok((in_port.clone(), midi_in))
    }
}
mod operator {
    use crate::adsr;
    use crate::configuration;
    use adsr::Adsr;
    use configuration::OperatorConfiguration;
    pub struct Operator {
        phase: f32,
        adsr: Adsr,
    }
    impl Operator {
        pub fn new() -> Self { Self { phase: 0.0, adsr: Adsr::new() } }
        pub fn reset(&mut self) { self.adsr.reset(); self.phase = 0.0; }
        pub fn tick(&mut self, modulation: f32,
            configuration: &OperatorConfiguration, phase_increment: f32,
            key_velocity: f32, note_on: bool) -> f32 {
            self.phase +=
                phase_increment * configuration.frequency_multiplier;
            if self.phase > std::f32::consts::PI * 2.0 {
                    self.phase -= std::f32::consts::PI * 2.0;
                }
            (self.phase + modulation).sin() *
                    self.adsr.tick(configuration, note_on) * key_velocity
        }
    }
}
mod oscilloscope {
    use macroquad::prelude::*;
    use crate::Samples;
    fn draw_my_cool_thingy(data: &Vec<f32>) {
        let x_step = screen_width() / data.len() as f32;
        for i in 0..data.len() - 1 {
            screen_width();
            screen_height();
            draw_line(x_step * i as f32,
                data[i] * screen_height() / 4.0 + screen_height() / 2.0,
                x_step * (i as f32 + 1.0),
                data[i + 1] * screen_height() / 4.0 + screen_height() / 2.0,
                2.0, WHITE);
        }
    }
    pub async fn ui_loop(samples: Samples) {
        loop {
            clear_background(BLACK);
            draw_my_cool_thingy(&samples.read().unwrap());
            next_frame().await
        }
    }
}
mod voice {
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
            let mut v =
                Self {
                    sample_rate: 44100,
                    base_frequency: 440.0,
                    phase_increment: 0.0,
                    key_velocity: 0.0,
                    note_on: false,
                    operators: <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([Operator::new()])),
                };
            v.set_frequency(440.0);
            v
        }
        pub fn note_on(&mut self, frequency: f32, velocity: f32) {
            self.set_frequency(frequency);
            self.key_velocity = velocity;
            self.note_on = true;
            for operator in self.operators.iter_mut() { operator.reset(); }
        }
        pub fn note_off(&mut self) { self.note_on = false; }
        pub fn set_frequency(&mut self, frequency: f32) {
            self.base_frequency = frequency;
            self.phase_increment =
                (2.0 * std::f32::consts::PI * frequency) /
                    self.sample_rate as f32;
        }
        pub fn tick(&mut self, configurations: &[OperatorConfiguration])
            -> f32 {
            let mut result = 0.0;
            for (operator, configuration) in
                self.operators.iter_mut().zip(configurations) {
                result +=
                    operator.tick(0.0, configuration, self.phase_increment,
                        self.key_velocity, self.note_on);
            }
            result
        }
    }
}
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
            voices: std::iter::repeat_with(Voice::new).take(8).collect(),
            voice_index: 0,
        }
    }
    fn note_on(&mut self, note: f32, velocity: f32) {
        self.voices[self.voice_index].note_on(note, velocity);
        self.voice_index += 1;
        if self.voice_index >= self.voices.len() { self.voice_index = 0; }
    }
    fn note_off(&mut self, note: f32) {
        for voice in self.voices.iter_mut() {
            if voice.base_frequency == note { voice.note_off() }
        }
    }
    fn tick(&mut self, configuration: &SynthConfiguration) -> f32 {
        self.voices.iter_mut().map(|voice|
                    voice.tick(configuration.operators_configuration.as_slice())
                        * 0.3).sum()
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
        result +=
            self.voice_manager.write().unwrap().tick(synth_configuration);
        let mut samples = self.samples.write().unwrap();
        samples[self.sample_index] = result;
        self.sample_index += 1;
        if self.sample_index >= samples.len() { self.sample_index = 0; }
        Some(result)
    }
}
impl Source for Synth {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { SAMPLE_RATE as u32 }
    fn total_duration(&self) -> Option<Duration> { None }
}
fn main() { macroquad::Window::new("BasicShapes", amain()); }
async fn amain() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let samples: Samples =
        Arc::new(RwLock::new(::alloc::vec::from_elem(0.0_f32, 1000)));
    let configuration = Arc::new(RwLock::new(SynthConfiguration::new()));
    let voice_manager = Arc::new(RwLock::new(VoiceManager::new()));
    let source =
        Synth {
            voice_manager: voice_manager.clone(),
            configuration: configuration.clone(),
            samples: samples.clone(),
            sample_index: 0,
        };
    sink.append(source);
    let (in_port, midi_in) = midi_input::midi_test().unwrap();
    let vm = voice_manager.clone();
    let _conn_in =
        midi_in.connect(&in_port, "midir-read-input",
            move |stamp, message, _|
                {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", ": ",
                                            " (len = ", ")\n"],
                                &[::core::fmt::ArgumentV1::new_display(&stamp),
                                            ::core::fmt::ArgumentV1::new_debug(&message),
                                            ::core::fmt::ArgumentV1::new_display(&message.len())]));
                    };
                    let frequency =
                        440.0 *
                            (2.0_f32).powf((message[1] as f32 - 69.0) as f32 / 12.0);
                    let event = LiveEvent::parse(message).unwrap();
                    match event {
                        LiveEvent::Midi { channel, message } =>
                            match message {
                                MidiMessage::NoteOn { key, vel } => {
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["hit note ",
                                                            ", ", " on channel ", "\n"],
                                                &[::core::fmt::ArgumentV1::new_display(&key),
                                                            ::core::fmt::ArgumentV1::new_display(&vel),
                                                            ::core::fmt::ArgumentV1::new_display(&channel)]));
                                    };
                                    if vel == 0 {
                                            voice_manager.write().unwrap().note_off(frequency);
                                        } else {
                                           let velocity: f32 = vel.as_int() as f32 / 127.0;
                                           voice_manager.write().unwrap().note_on(frequency, velocity);
                                       }
                                }
                                MidiMessage::NoteOff { key, vel } => {
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["note off ",
                                                            ", ", " on channel ", "\n"],
                                                &[::core::fmt::ArgumentV1::new_display(&key),
                                                            ::core::fmt::ArgumentV1::new_display(&vel),
                                                            ::core::fmt::ArgumentV1::new_display(&channel)]));
                                    };
                                    voice_manager.write().unwrap().note_off(frequency);
                                }
                                MidiMessage::Controller { controller, value } => {
                                    {
                                        ::std::io::_print(::core::fmt::Arguments::new_v1(&["control change ",
                                                            ", ", " on channel ", "\n"],
                                                &[::core::fmt::ArgumentV1::new_display(&controller),
                                                            ::core::fmt::ArgumentV1::new_display(&value),
                                                            ::core::fmt::ArgumentV1::new_display(&channel)]));
                                    };
                                }
                                _ => {}
                            },
                        _ => {
                            {
                                ::std::io::_print(::core::fmt::Arguments::new_v1(&["No idea what this is\n"],
                                        &[]));
                            }
                        }
                    }
                    if message[2] == 0 { return; }
                }, ());
    let voice_manager = vm;
    let ui = async { oscilloscope::ui_loop(samples.clone()).await; };
    ui.await;
    loop {
        let mut input = String::new();
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(&["Enter number: \n"],
                    &[]));
        };
        io::stdin().read_line(&mut input).expect("Not a valid string");
        if let Ok(num) = input.trim().parse::<f32>() {
                voice_manager.write().unwrap().note_on(num, 0.8);
                if num < 0.0 {}
            } else {
               {
                   ::std::io::_print(::core::fmt::Arguments::new_v1(&["Invalid number\n"],
                           &[]));
               };
               break;
           }
    }
    sink.sleep_until_end();
}
