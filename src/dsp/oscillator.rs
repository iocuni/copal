use std::f32::consts::TAU;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

pub struct Oscillator {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
}

impl Oscillator {
    pub fn new() -> Self {
        Self {
            phase: 0.0,
            frequency: 440.0,
            sample_rate: 44_100.0,
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn reset_phase(&mut self) {
        self.phase = 0.0;
    }

    pub fn next_sample(&mut self, waveform: Waveform) -> f32 {
        let sample = match waveform {
            Waveform::Sine => (self.phase * TAU).sin(),
            Waveform::Square => {
                if self.phase < 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Saw => 2.0 * self.phase - 1.0,
            Waveform::Triangle => 1.0 - 4.0 * (self.phase - 0.5).abs(),
        };

        self.phase = (self.phase + self.frequency / self.sample_rate).fract();
        sample
    }
}

#[cfg(test)]
mod tests {
    use super::{Oscillator, Waveform};

    #[test]
    fn sine_wave_advances_one_cycle_at_the_configured_frequency() {
        let mut oscillator = Oscillator::new();
        oscillator.set_sample_rate(4.0);
        oscillator.set_frequency(1.0);

        assert!(oscillator.next_sample(Waveform::Sine).abs() < 1e-6);
        assert!((oscillator.next_sample(Waveform::Sine) - 1.0).abs() < 1e-6);
        assert!(oscillator.next_sample(Waveform::Sine).abs() < 1e-6);
        assert!((oscillator.next_sample(Waveform::Sine) + 1.0).abs() < 1e-6);
    }

    #[test]
    fn saw_wave_wraps_from_positive_to_negative() {
        let mut oscillator = Oscillator::new();
        oscillator.set_sample_rate(4.0);
        oscillator.set_frequency(1.0);

        assert_eq!(oscillator.next_sample(Waveform::Saw), -1.0);
        assert_eq!(oscillator.next_sample(Waveform::Saw), -0.5);
        assert_eq!(oscillator.next_sample(Waveform::Saw), 0.0);
        assert_eq!(oscillator.next_sample(Waveform::Saw), 0.5);
    }
}
