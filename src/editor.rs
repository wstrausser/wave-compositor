use std::sync::Arc;
use std::time::Duration;

use nih_plug::prelude::*;
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;

use crate::plugin::WaveCompositorParams;

pub fn default_state() -> Arc<IcedState> {
    IcedState::from_size(800, 600)
}

pub fn create(
    params: Arc<WaveCompositorParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<WaveCompositorEditor>(editor_state, (params))
}

struct WaveCompositorEditor {
    params: Arc<WaveCompositorParams>,
    context: Arc<dyn GuiContext>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for WaveCompositorEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<WaveCompositorParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = WaveCompositorEditor { params, context };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .align_items(Alignment::Center)
            .push(Text::new("Wave Compositor"))
            .into()
    }
}
