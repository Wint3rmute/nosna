use crate::adsr;
use crate::configuration;
// use crate::adsr;
use adsr::Adsr;
use configuration::OperatorConfiguration;

pub struct Operator {
    phase: f32,
    adsr: Adsr,
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
        phase_increment: f32,
        key_velocity: Option<f32>,
    ) -> f32 {
        self.phase += phase_increment * configuration.frequency_multiplier;

        if self.phase > std::f32::consts::PI * 2.0 {
            self.phase -= std::f32::consts::PI * 2.0;
        }

        (self.phase + modulation).sin() * self.adsr.tick(configuration, key_velocity)
    }
}
