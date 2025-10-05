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
            Waveform::Sine => self.sample_sine(),
            Waveform::Saw => self.sample_saw(),
            Waveform::Square => self.sample_square(),
            Waveform::Triangle => self.sample_triangle(),
        };

        self.increment_phase(frequency, sample_rate);

        let scaled_sample = new_sample * gain;

        if gain > 0.1 {
            println!("frequency: {}, gain: {}, sample_rate: {}, sample: {}", frequency, gain, sample_rate, scaled_sample);
        }

        scaled_sample
    }

    fn increment_phase(&mut self, frequency: f32, sample_rate: f32) {
        let phase_delta = frequency / sample_rate;

        self.phase += phase_delta;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
    }

    fn sample_sine(&mut self) -> f32 {
        (self.phase * consts::TAU).sin()
    }

    fn sample_saw(&mut self) -> f32 {
        (self.phase * 2.0) - 1.0
    }

    fn sample_square(&mut self) -> f32 {
        if self.phase >= 0.5 { 1.0 } else { -1.0 }
    }

    fn sample_triangle(&mut self) -> f32 {
        if self.phase >= 0.5 {
            let phase_inverse = self.phase - 0.5;

            1.0 - (phase_inverse * 4.0)
        } else {
            -1.0 + (self.phase * 4.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;

    const FREQUENCY: f32 = 1.0;
    const SAMPLE_RATE: f32 = 8.0;

    fn run_test(waveform: Waveform, expected: Vec<f32>) {
        let actual = {
            let mut wave = Wave::new(waveform);
            let mut buf: Vec<f32> = Vec::new();
            for _ in 0..expected.len() {
                let new_sample = wave.sample(FREQUENCY, 1.0, SAMPLE_RATE);
                buf.push(new_sample);
            }
            buf
        };

        let mut is_pass = true;

        for i in 0..expected.len() {
            let left = expected[i];
            let right = actual[i];

            println!("{:.3} = {:3}", left, right);

            if !approx_eq!(f32, left, right, epsilon = 0.001) {
                is_pass = false;
            };
        }

        assert!(is_pass)
    }

    #[test]
    fn test_sine() {
        let expected = vec![0.0, 0.707, 1.0, 0.707, 0.0, -0.707, -1.0, -0.707];

        run_test(Waveform::Sine, expected)
    }

    #[test]
    fn test_saw() {
        let expected = vec![-1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75];

        run_test(Waveform::Saw, expected)
    }

    #[test]
    fn test_square() {
        let expected = vec![-1.0, -1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0];

        run_test(Waveform::Square, expected)
    }

    #[test]
    fn test_triangle() {
        let expected = vec![-1.0, -0.5, 0.0, 0.5, 1.0, 0.5, 0.0, -0.5];

        run_test(Waveform::Triangle, expected)
    }
}
