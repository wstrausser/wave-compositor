use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

use crate::editor;
use crate::wave::Wave;

pub struct WaveCompositor {
    params: Arc<WaveCompositorParams>,
    sample_rate: f32,
    wave_1: Wave,
    wave_2: Wave,
    wave_3: Wave,
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

#[derive(Clone, Debug, Enum, PartialEq)]
pub enum Waveform {
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
            sample_rate: 1.0,
            wave_1: Wave::new(Waveform::Sine),
            wave_2: Wave::new(Waveform::Sine),
            wave_3: Wave::new(Waveform::Sine),
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
                FloatRange::Linear {
                    min: 260.0,
                    max: 520.0,
                },
            )
            .with_unit(" hz"),
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
                FloatRange::Skewed {
                    min: 0.1,
                    max: 10.0,
                    factor: 0.25,
                },
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
                FloatRange::Linear {
                    min: -0.1,
                    max: 0.1,
                },
            ),
        }
    }
}

impl Plugin for WaveCompositor {
    const NAME: &'static str = "Wave Compositor (v0.1.1)";

    const VENDOR: &'static str = "William Strausser";

    const URL: &'static str = "https://github.com/wstrausser/sine-compositor/";

    const EMAIL: &'static str = "william.e.strausser@gmail.com";

    const VERSION: &'static str = "0.1.1";

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
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let selected_waveform = self.params.waveform.value();

        if self.wave_1.waveform != selected_waveform {
            self.wave_1 = Wave::new(selected_waveform.clone());
            self.wave_2 = Wave::new(selected_waveform.clone());
            self.wave_3 = Wave::new(selected_waveform.clone());
        }

        for (_, channel_samples) in buffer.iter_samples().enumerate() {
            for sample in channel_samples {
                let wave_1_sample = self.wave_1.sample(
                    (self.params.base_frequency.smoothed.next()
                        * self.params.wave_1.multiplier.smoothed.next())
                        * (1.0 + self.params.wave_1.offset.smoothed.next()),
                    self.params.wave_1.gain.smoothed.next(),
                    self.sample_rate,
                );
                let wave_2_sample = self.wave_2.sample(
                    (self.params.base_frequency.smoothed.next()
                        * self.params.wave_2.multiplier.smoothed.next())
                        * (1.0 + self.params.wave_2.offset.smoothed.next()),
                    self.params.wave_2.gain.smoothed.next(),
                    self.sample_rate,
                );
                let wave_3_sample = self.wave_3.sample(
                    (self.params.base_frequency.smoothed.next()
                        * self.params.wave_3.multiplier.smoothed.next())
                        * (1.0 + self.params.wave_3.offset.smoothed.next()),
                    self.params.wave_3.gain.smoothed.next(),
                    self.sample_rate,
                );

                *sample = wave_1_sample + wave_2_sample + wave_3_sample;
            }
        }
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for WaveCompositor {
    const VST3_CLASS_ID: [u8; 16] = *b"WaveCompositor  ";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Synth];
}
