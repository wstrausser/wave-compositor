use nih_plug::prelude::nih_export_vst3;

use crate::plugin::WaveCompositor;

pub mod editor;
pub mod plugin;
pub mod wave;


nih_export_vst3!(WaveCompositor);
