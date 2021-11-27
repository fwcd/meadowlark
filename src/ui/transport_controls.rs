use vizia::*;

use crate::state::{AppEvent, StateSystem, ui_state::{TimelineTransportUiState, UiState}};

pub fn transport_controls(cx: &mut Context) {

    VStack::new(cx, |cx|{
        Label::new(cx, "TRANSPORT");
        HStack::new(cx, |cx|{

            Binding::new(cx, StateSystem::ui_state.then(UiState::timeline_transport).then(TimelineTransportUiState::playhead), |cx, playhead|{
                let beats = playhead.get(cx).0;
                Label::new(cx, &format!("{}.{}.{}",  (beats/ 4.0) as i32 + 1, beats as i32 + 1, ((beats * 4.0) as i32 % 4) + 1));
            });

            let init = (130.0 - 20.0) / 180.0;
            Binding::new(cx, StateSystem::ui_state.then(UiState::timeline_transport).then(TimelineTransportUiState::is_playing), |cx, is_playing|{
                HStack::new(cx, move |cx|{
                    Label::new(cx, "PLAY/PAUSE: ").width(Pixels(120.0));
                    Checkbox::new(cx, *is_playing.get(cx))
                        .on_checked(cx, |cx| cx.emit(AppEvent::Play))
                        .on_unchecked(cx, |cx| cx.emit(AppEvent::Pause))
                        .top(Stretch(1.0))               
                        .bottom(Stretch(1.0));                
                });
            });

            Button::new(cx, |cx| cx.emit(AppEvent::Stop), |cx|{
                Label::new(cx, "STOP");
            });
        }).height(Pixels(30.0));
    }).child_space(Pixels(10.0));
}