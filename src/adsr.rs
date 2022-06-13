use crate::configuration::OperatorConfiguration;
use crate::voice::Voice;
use std::sync::Arc;

pub enum Phase {
    Attack,
    Decay,
    Sustain,
    Release,
    Finished,
}

pub struct Adsr {
    phase: Phase,
    state: f32,
}

impl Adsr {
    pub fn tick(
        &mut self,
        configuration: &OperatorConfiguration,
        key_velocity: Option<f32>,
    ) -> f32 {
        if let Some(velocity) = key_velocity {
            match self.phase {
                Phase::Attack => {
                    self.state += configuration.attack;
                    if self.state > 1.0 {
                        self.state = 1.0;
                        self.phase = Phase::Decay;
                        println!("Attack DONE");
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
                        println!("Adsr:Finished");
                    }
                }
                _ => {
                    self.phase = Phase::Release;
                }
            }
        }
        self.state
    }

    pub fn reset(&mut self) {
        self.state = 0.0;
        self.phase = Phase::Attack;
    }

    pub fn new() -> Self {
        Self {
            state: 0.0,
            phase: Phase::Attack,
        }
    }
}
