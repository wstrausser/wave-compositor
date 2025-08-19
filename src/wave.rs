use std::f32::consts;

use crate::plugin::Waveform;

pub struct Wave {
    pub waveform: Waveform,
    phase: f32,
}

impl Wave {
    pub fn new(waveform: Waveform) -> Self {
        Wave {
            waveform: waveform,
            phase: 0.0,
        }
    }

    pub fn sample(&mut self, frequency: f32, gain: f32, sample_rate: f32) -> f32 {
        let new_sample = match self.waveform {
            Waveform::Sine => {
                self.sample_sine(frequency, sample_rate)
            },
            Waveform::Saw => {
                self.sample_saw(frequency, sample_rate)
            },
            Waveform::Square => {
                self.sample_square(frequency, sample_rate)
            },
            Waveform::Triangle => {
                self.sample_triangle(frequency, sample_rate)
            }
        };

        new_sample * gain
    }

    fn sample_sine(&mut self, frequency: f32, sample_rate: f32) -> f32 {
        let phase_delta = frequency / sample_rate;

        let sine = (self.phase * consts::TAU).sin();

        self.phase += phase_delta;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sine
    }

    fn sample_saw(&mut self, frequency: f32, sample_rate: f32) -> f32 {
        let phase_delta = frequency / sample_rate;

        self.phase += phase_delta;

        if self.phase >= 1.0 {
            self.phase -= 2.0;
        }

        self.phase
    }

    fn sample_square(&mut self, frequency: f32, sample_rate: f32) -> f32 {
        let phase_delta = frequency / sample_rate;
        
        self.phase += phase_delta;

        if self.phase >= 1.0 {
            self.phase -= 2.0
        }

        return if self.phase >= 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    fn sample_triangle(&mut self, frequency: f32, sample_rate: f32) -> f32 {
        let phase_delta = frequency / sample_rate;

        self.phase += phase_delta;

        if self.phase >= 1.0 {
            self.phase -= 2.0
        }

        return if self.phase >= 0.0 {
            1.0 - (self.phase * 2.0)
        } else {
            -1.0 + ((1.0 - self.phase.abs()) * 2.0)
        }

    }
}
