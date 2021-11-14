use vizia::*;

use crate::state::{BoundGuiState, event::{StateSystemEvent, TransportEvent}};

/// Widget for the TEMPO control bar
#[derive(Default)]
pub struct TransportControlBar {}

impl TransportControlBar {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self{}.build(cx)
    }
}

impl View for TransportControlBar {
    fn body(&mut self, cx: &mut Context) {
        //let controls = ControlBar::new("TRANSPORT").build(state, entity, |builder| builder);

        // Playhead position
        Label::new(cx, "5.2.3");
            //.bind(AppData::beats_per_minute, |value| value.to_string())
            //.build(state, controls, |builder| builder.set_name("playhead position"));

        // Play/ Pause button

        Binding::new(cx, BoundGuiState::is_playing, |cx, is_playing|{
            //let is_playing = is_playing.get(cx);
            HStack::new(cx, move |cx|{
                Checkbox::new(cx, *is_playing.get(cx))
                    .on_checked(cx, |cx| cx.emit(StateSystemEvent::Transport(TransportEvent::Play)))
                    .on_unchecked(cx, |cx| cx.emit(StateSystemEvent::Transport(TransportEvent::Pause)));
    
                
                Checkbox::new(cx, *is_playing.get(cx))
                    .on_checked(cx, |cx| cx.emit(StateSystemEvent::Transport(TransportEvent::Play)))
                    .on_unchecked(cx, |cx| cx.emit(StateSystemEvent::Transport(TransportEvent::Pause)));

            });
        });

        // CheckButton::new()
        //     .on_checked(|data, state, checkbutton| {
        //         checkbutton.set_text(state, "PAUSE");
        //         checkbutton.emit(state, TransportEvent::Play.to_state_event());
        //     })
        //     .on_unchecked(|data, state, checkbutton| {
        //         checkbutton.set_text(state, "PLAY");
        //         checkbutton.emit(state, TransportEvent::Pause.to_state_event());
        //     })
        //     .bind(BoundGuiState::is_playing, |is_playing| *is_playing)
        //     .build(state, controls, |builder| builder);

        // // Stop button
        // Button::with_label("STOP")
        //     .on_press(|_, state, button| {
        //         button.emit(state, TransportEvent::Stop.to_state_event());
        //     })
        //     .bind(BoundGuiState::is_playing, |data| ())
        //     .build(state, controls, |builder| builder);

        //entity.class(state, "control_bar").set_name(state, "transport controls")
    }
}
