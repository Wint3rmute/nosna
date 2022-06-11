use crate::adsr;
use crate::configuration;
// use crate::adsr;
use adsr::ADSR;
use configuration::{OperatorConfiguration, VoiceState};

pub struct Operator {
    phase: f32,
    adsr: ADSR,
}

impl Operator {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            adsr: ADSR::new(),
        }
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

        (self.phase + modulation).sin() * self.adsr.tick(configuration, voice_state)
    }
}
