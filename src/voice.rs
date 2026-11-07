use crate::{
    dsp::{envelope::Envelope, filter::LowPassFilter, oscillator::Oscillator},
    params::SynthParams,
    util::midi::note_to_frequency,
};

pub struct Voice {
    note: Option<u8>,
    velocity: f32,
    age: u64,
    oscillator: Oscillator,
    envelope: Envelope,
    filter: LowPassFilter,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        let mut oscillator = Oscillator::new();
        oscillator.set_sample_rate(sample_rate);

        Self {
            note: None,
            velocity: 0.0,
            age: 0,
            oscillator,
            envelope: Envelope::new(),
            filter: LowPassFilter::new(),
        }
    }

    pub fn note_on(&mut self, note: u8, velocity: f32, age: u64) {
        self.note = Some(note);
        self.velocity = velocity;
        self.age = age;
        self.oscillator.set_frequency(note_to_frequency(note));
        self.oscillator.reset_phase();
        self.envelope.trigger();
        self.filter.reset();
    }

    pub fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None;
            self.envelope.release();
        }
    }

    pub fn is_active(&self) -> bool {
        self.note.is_some() || !self.envelope.is_idle()
    }

    pub fn is_releasing(&self) -> bool {
        self.note.is_none() && !self.envelope.is_idle()
    }

    pub fn age(&self) -> u64 {
        self.age
    }

    pub fn next_sample(&mut self, params: &SynthParams, sample_rate: f32) -> f32 {
        let envelope = self.envelope.next_sample(
            params.attack.value(),
            params.decay.value(),
            params.sustain.value(),
            params.release.value(),
            sample_rate,
        );
        if self.envelope.is_idle() {
            self.velocity = 0.0;
            return 0.0;
        }

        let sample =
            self.oscillator.next_sample(params.waveform.value().into()) * self.velocity * envelope;
        self.filter
            .process(sample, params.cutoff.value(), sample_rate)
    }
}
