use std::f32::consts::TAU;

pub struct LowPassFilter {
    state: f32,
}

impl LowPassFilter {
    pub fn new() -> Self {
        Self { state: 0.0 }
    }

    pub fn reset(&mut self) {
        self.state = 0.0;
    }

    pub fn process(&mut self, input: f32, cutoff: f32, sample_rate: f32) -> f32 {
        let cutoff = cutoff.clamp(1.0, sample_rate * 0.49);
        let coefficient = 1.0 - (-TAU * cutoff / sample_rate).exp();
        self.state += coefficient * (input - self.state);
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::LowPassFilter;

    #[test]
    fn low_pass_filter_moves_toward_a_constant_input() {
        let mut filter = LowPassFilter::new();
        let first = filter.process(1.0, 1_000.0, 48_000.0);
        let later = filter.process(1.0, 1_000.0, 48_000.0);

        assert!(first > 0.0);
        assert!(later > first);
        assert!(later < 1.0);
    }
}
