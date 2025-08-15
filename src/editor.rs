use std::sync::Arc;

use nih_plug_iced::IcedState;

use crate::plugin::WaveCompositorParams;


pub fn default_state() -> Arc<IcedState> {
    IcedState::from_size(800, 600)
}
