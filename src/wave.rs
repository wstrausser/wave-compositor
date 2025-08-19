use std::f32::consts;

use crate::plugin::Waveform;

pub struct Wave {
    waveform: Waveform,
    sample_rate: f32,
    phase: f32,
}

impl Wave {
    pub fn new(waveform: Waveform) -> Self {
        Wave {
            waveform: waveform,
            sample_rate: 44100.0,
            phase: 0.0,
        }
    }

    pub fn sample(&mut self, frequency: f32, gain: f32) -> f32 {
        let new_sample = match self.waveform {
            Waveform::Sine => {
                self.sample_sine(frequency)
            },
            Waveform::Saw => {
                self.sample_saw(frequency)
            },
            Waveform::Square => {
                self.sample_square(frequency)
            },
            Waveform::Triangle => {
                self.sample_triangle(frequency)
            }
        };

        new_sample * gain
    }

    fn sample_sine(&mut self, frequency: f32) -> f32 {
        let phase_delta = frequency / self.sample_rate;

        let sine = (self.phase * consts::TAU).sin();

        self.phase += phase_delta;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sine
    }

    fn sample_saw(&mut self, frequency: f32) -> f32 {
        todo!()
    }

    fn sample_square(&mut self, frequency: f32) -> f32 {
        todo!()
    }

    fn sample_triangle(&mut self, frequency: f32) -> f32 {
        todo!()
    }
}
