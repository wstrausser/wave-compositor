use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

use crate::editor;


pub struct WaveCompositor {
    params: Arc<WaveCompositorParams>,
}

#[derive(Params)]
pub struct WaveCompositorParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    #[id = "waveform"]
    waveform: EnumParam<Waveform>,

    #[id = "base_frequency"]
    base_frequency: FloatParam,

    #[nested(id_prefix = "wave-1")]
    wave_1: WaveParams,

    #[nested(id_prefix = "wave-2")]
    wave_2: WaveParams,

    #[nested(id_prefix = "wave-3")]
    wave_3: WaveParams,
}

#[derive(Params)]
struct WaveParams {
    #[id = "multiplier"]
    multiplier: FloatParam,

    #[id = "gain"]
    gain: FloatParam,

    #[id = "offset"]
    offset: FloatParam,
}

#[derive(Enum, Debug, PartialEq)]
enum Waveform {
    #[id = "sine"]
    Sine,

    #[id = "saw"]
    Saw,

    #[id = "square"]
    Square,

    #[id = "triangle"]
    Triangle,
}


impl Default for WaveCompositor {
    fn default() -> Self {
        Self {
            params: Arc::new(WaveCompositorParams::default()),
        }
    }
}


impl Default for WaveCompositorParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            waveform: EnumParam::new("Waveform", Waveform::Sine),
            base_frequency: FloatParam::new(
                "Base Frequency",
                440.0,
                FloatRange::Linear { min: 260.0, max: 520.0 },
            ).with_unit(" hz"),
            wave_1: WaveParams::default(),
            wave_2: WaveParams::default(),
            wave_3: WaveParams::default(),
        }
    }
}


impl Default for WaveParams {
    fn default() -> Self {
        Self {
            multiplier: FloatParam::new(
                "Multiplier",
                1.0,
                FloatRange::Skewed { min: 0.1, max: 10.0, factor: 0.25 },
            ),
            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(10.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 10.0),
                },
            ),
            offset: FloatParam::new(
                "Offset",
                0.0,
                FloatRange::Linear { min: -0.1, max: 0.1 },
            ),
        }
    }
}


impl Plugin for WaveCompositor {
    const NAME: &'static str = "Wave Compositor";
    
    const VENDOR: &'static str = "William Strausser";
    
    const URL: &'static str = "https://github.com/wstrausser/sine-compositor/";
    
    const EMAIL: &'static str = "william.e.strausser@gmail.com";
    
    const VERSION: &'static str = "0.1.0";
    
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];
    
    type SysExMessage = ();
    
    type BackgroundTask = ();
    
    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }
    
    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for WaveCompositor {
    const VST3_CLASS_ID: [u8; 16] = *b"WaveCompositor  ";
    
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Synth];
}
