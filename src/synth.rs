pub enum SynthParam {
    A1(u8),
    A2(u8),
    A3(u8),
    A4(u8),
    B1(u8),
    B2(u8),
    B3(u8),
    B4(u8),
}

pub trait Synth {
    fn next() -> f32;
    fn set_param(param: SynthParam);

    fn param_to_name(param: SynthParam) -> str;

    fn note_on();
    fn note_off();
}

struct WaveTableSynth {}

// impl Synth for WaveTableSynth {}
