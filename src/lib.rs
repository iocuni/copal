mod dsp;
mod params;
mod plugin;
mod synth;
mod util;
mod voice;

use nih_plug::prelude::*;

nih_export_vst3!(plugin::CopalPlugin);
nih_export_clap!(plugin::CopalPlugin);
