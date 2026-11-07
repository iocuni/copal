use std::sync::Arc;

use nih_plug::prelude::*;

use crate::{params::SynthParams, synth::Synth};

pub struct CopalPlugin {
    params: Arc<SynthParams>,
    synth: Synth,
}

impl Default for CopalPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(SynthParams::default()),
            synth: Synth::new(44_100.0),
        }
    }
}

impl Plugin for CopalPlugin {
    const NAME: &'static str = "Copal";
    const VENDOR: &'static str = "Copal Audio";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: Some(new_nonzero_u32(2)),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.synth.set_sample_rate(buffer_config.sample_rate);
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();

        for (sample_index, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() as usize > sample_index {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } if velocity > 0.0 => {
                        self.synth.note_on(note, velocity)
                    }
                    NoteEvent::NoteOn { note, .. } | NoteEvent::NoteOff { note, .. } => {
                        self.synth.note_off(note)
                    }
                    _ => {}
                }

                next_event = context.next_event();
            }

            let sample = self.synth.next_sample(&self.params);
            for channel_sample in channel_samples {
                *channel_sample = sample;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for CopalPlugin {
    const CLAP_ID: &'static str = "com.copal.audio";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A simple polyphonic synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] =
        &[ClapFeature::Instrument, ClapFeature::Synthesizer];
}

impl Vst3Plugin for CopalPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"CopalSimpleSynth";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}
