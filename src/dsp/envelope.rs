#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct Envelope {
    stage: Stage,
    value: f32,
    release_start: f32,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            stage: Stage::Idle,
            value: 0.0,
            release_start: 0.0,
        }
    }

    pub fn trigger(&mut self) {
        self.value = 0.0;
        self.release_start = 0.0;
        self.stage = Stage::Attack;
    }

    pub fn release(&mut self) {
        if self.stage != Stage::Idle && self.stage != Stage::Release {
            self.release_start = self.value;
            self.stage = Stage::Release;
        }
    }

    pub fn is_idle(&self) -> bool {
        self.stage == Stage::Idle
    }

    pub fn next_sample(
        &mut self,
        attack: f32,
        decay: f32,
        sustain: f32,
        release: f32,
        sample_rate: f32,
    ) -> f32 {
        match self.stage {
            Stage::Idle => self.value = 0.0,
            Stage::Attack => {
                if attack <= 0.0 {
                    self.value = 1.0;
                } else {
                    self.value = (self.value + 1.0 / (attack * sample_rate)).min(1.0);
                }
                if self.value >= 1.0 {
                    self.stage = Stage::Decay;
                }
            }
            Stage::Decay => {
                if decay <= 0.0 {
                    self.value = sustain;
                } else {
                    self.value =
                        (self.value - (1.0 - sustain) / (decay * sample_rate)).max(sustain);
                }
                if self.value <= sustain {
                    self.stage = Stage::Sustain;
                }
            }
            Stage::Sustain => self.value = sustain,
            Stage::Release => {
                if release <= 0.0 || self.release_start <= 0.0 {
                    self.value = 0.0;
                } else {
                    self.value =
                        (self.value - self.release_start / (release * sample_rate)).max(0.0);
                }
                if self.value <= 0.0 {
                    self.stage = Stage::Idle;
                }
            }
        }

        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::Envelope;

    #[test]
    fn envelope_follows_adsr_stages() {
        let mut envelope = Envelope::new();
        envelope.trigger();

        assert_eq!(envelope.next_sample(0.0, 0.0, 0.5, 0.0, 1.0), 1.0);
        assert_eq!(envelope.next_sample(0.0, 0.0, 0.5, 0.0, 1.0), 0.5);

        envelope.release();
        assert_eq!(envelope.next_sample(0.0, 0.0, 0.5, 0.0, 1.0), 0.0);
        assert!(envelope.is_idle());
    }
}
