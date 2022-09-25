use crate::adsr;
use crate::configuration;
use crate::voice;
// use crate::adsr;
use adsr::Adsr;
use configuration::OperatorConfiguration;
use voice::VoiceState;

pub struct Operator {
    pub phase: f32,
    pub adsr: Adsr,
}

impl Operator {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            adsr: Adsr::new(),
        }
    }

    pub fn reset(&mut self) {
        self.adsr.reset();
        self.phase = 0.0;
    }

    pub fn tick(
        &mut self,
        modulation: f32,
        configuration: &OperatorConfiguration,
        voice_state: &VoiceState,
    ) -> f32 {
        self.phase += voice_state.phase_increment * configuration.frequency_multiplier;

        if self.phase > std::f32::consts::PI * 2.0 {
            self.phase -= std::f32::consts::PI * 2.0;
        }

        (self.phase + modulation).sin()
            * self.adsr.tick(configuration, voice_state.note_on)
            * voice_state.key_velocity
            * configuration.strength
    }
}
