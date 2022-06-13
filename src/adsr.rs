use crate::configuration::OperatorConfiguration;
use crate::voice::Voice;
use std::sync::Arc;

pub enum Phase {
    ATTACK,
    DECAY,
    SUSTAIN,
    RELEASE,
    FINISHED,
}

pub struct ADSR {
    phase: Phase,
    state: f32,
}

impl ADSR {
    pub fn tick(&mut self, configuration: &OperatorConfiguration, voice_state: &Voice) -> f32 {
        if let Some(velocity) = voice_state.key_velocity {
            match self.phase {
                Phase::ATTACK => {
                    self.state += configuration.attack;
                    if self.state > 1.0 {
                        self.state = 1.0;
                        self.phase = Phase::DECAY;
                        println!("ATTACK DONE");
                    }
                }
                Phase::DECAY => {
                    self.state -= configuration.decay;
                    if self.state < configuration.sustain {
                        self.phase = Phase::SUSTAIN;
                        self.state = configuration.sustain;
                    }
                }
                Phase::SUSTAIN => {}
                Phase::RELEASE | Phase::FINISHED => {
                    self.state = 0.0;
                    self.phase = Phase::ATTACK
                }
            }
        } else {
            match self.phase {
                Phase::FINISHED => {}
                Phase::RELEASE => {
                    self.state -= configuration.release;
                    if self.state < 0.0 {
                        self.state = 0.0;
                        self.phase = Phase::FINISHED;
                        println!("ADSR FINISHED");
                    }
                }
                _ => {
                    self.phase = Phase::RELEASE;
                }
            }
        }
        self.state
    }

    pub fn reset(&mut self) {
        self.state = 0.0;
        self.phase = Phase::ATTACK;
    }

    pub fn new() -> Self {
        Self {
            state: 0.0,
            phase: Phase::ATTACK,
        }
    }
}
