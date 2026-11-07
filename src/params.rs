use nih_plug::prelude::*;

use crate::dsp::oscillator::Waveform as OscillatorWaveform;

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Waveform {
    #[name = "Sine"]
    Sine,
    #[name = "Square"]
    Square,
    #[name = "Saw"]
    Saw,
    #[name = "Triangle"]
    Triangle,
}

impl From<Waveform> for OscillatorWaveform {
    fn from(waveform: Waveform) -> Self {
        match waveform {
            Waveform::Sine => Self::Sine,
            Waveform::Square => Self::Square,
            Waveform::Saw => Self::Saw,
            Waveform::Triangle => Self::Triangle,
        }
    }
}

#[derive(Params)]
pub struct SynthParams {
    #[id = "waveform"]
    pub waveform: EnumParam<Waveform>,
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "attack"]
    pub attack: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "sustain"]
    pub sustain: FloatParam,
    #[id = "release"]
    pub release: FloatParam,
    #[id = "cutoff"]
    pub cutoff: FloatParam,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            waveform: EnumParam::new("Waveform", Waveform::Sine),
            gain: FloatParam::new("Gain", 0.2, FloatRange::Linear { min: 0.0, max: 1.0 }),
            attack: FloatParam::new(
                "Attack",
                0.01,
                FloatRange::Skewed {
                    min: 0.001,
                    max: 2.0,
                    factor: 0.3,
                },
            )
            .with_unit(" s"),
            decay: FloatParam::new(
                "Decay",
                0.15,
                FloatRange::Skewed {
                    min: 0.001,
                    max: 3.0,
                    factor: 0.3,
                },
            )
            .with_unit(" s"),
            sustain: FloatParam::new("Sustain", 0.7, FloatRange::Linear { min: 0.0, max: 1.0 }),
            release: FloatParam::new(
                "Release",
                0.2,
                FloatRange::Skewed {
                    min: 0.001,
                    max: 5.0,
                    factor: 0.3,
                },
            )
            .with_unit(" s"),
            cutoff: FloatParam::new(
                "Filter Cutoff",
                12_000.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20_000.0,
                    factor: 0.25,
                },
            )
            .with_unit(" Hz"),
        }
    }
}
