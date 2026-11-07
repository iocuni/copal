use crate::{params::SynthParams, voice::Voice};

const VOICE_COUNT: usize = 8;

pub struct Synth {
    voices: [Voice; VOICE_COUNT],
    sample_rate: f32,
    next_age: u64,
}

impl Synth {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            voices: std::array::from_fn(|_| Voice::new(sample_rate)),
            sample_rate,
            next_age: 0,
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.voices = std::array::from_fn(|_| Voice::new(sample_rate));
    }

    pub fn note_on(&mut self, note: u8, velocity: f32) {
        let voice_index = self
            .voices
            .iter()
            .position(|voice| !voice.is_active())
            .or_else(|| {
                self.voices
                    .iter()
                    .enumerate()
                    .filter(|(_, voice)| voice.is_releasing())
                    .min_by_key(|(_, voice)| voice.age())
                    .map(|(index, _)| index)
            })
            .unwrap_or_else(|| {
                self.voices
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, voice)| voice.age())
                    .map(|(index, _)| index)
                    .expect("synth always has at least one voice")
            });

        self.next_age = self.next_age.wrapping_add(1);
        self.voices[voice_index].note_on(note, velocity, self.next_age);
    }

    pub fn note_off(&mut self, note: u8) {
        for voice in &mut self.voices {
            voice.note_off(note);
        }
    }

    pub fn next_sample(&mut self, params: &SynthParams) -> f32 {
        let sample = self
            .voices
            .iter_mut()
            .map(|voice| voice.next_sample(params, self.sample_rate))
            .sum::<f32>();
        sample * params.gain.value()
    }
}
