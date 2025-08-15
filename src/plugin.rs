pub struct WaveCompositorParams {
    waveform: Waveform,
    base_frequency: f32,
    wave_1: WaveParams,
    wave_2: WaveParams,
    wave_3: WaveParams,
}

struct WaveParams {
    multiplier: f32,
    gain: f32,
    offset: f32,    
}

enum Waveform {
    Sine,
    Saw,
    Square,
    Triangle,
}
