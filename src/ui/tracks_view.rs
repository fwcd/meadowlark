


use rusty_daw_core::MusicalTime;
use vizia::*;

use crate::state::{StateSystem, ui_state::{TimelineTransportUiState, UiState}};

pub fn tracks_view(cx: &mut Context) {

    if cx.data::<TracksViewState>().is_none() {
        // Create some internal slider data (not exposed to the user)
        TracksViewState {
            start_time: MusicalTime::new(0.into()),
            end_time: MusicalTime::new(40.into()),
        }.build(cx);
    }

    ZStack::new(cx, |cx|{
        // Background
        Element::new(cx).background_color(Color::rgb(76,68,69));
        // Playhead
        Binding::new(cx, TracksViewState::root, |cx, track_view_state|{
            let start_beats = track_view_state.get(cx).start_time;
            let end_beats = track_view_state.get(cx).end_time;
            Binding::new(cx, StateSystem::ui_state.then(UiState::timeline_transport).then(TimelineTransportUiState::playhead), move |cx, playhead|{
                let current_beats = playhead.get(cx);
                
                let should_display = current_beats.0 >= start_beats.0 && current_beats.0 <= end_beats.0;

                let mut ratio = (current_beats.0 - start_beats.0) / (end_beats.0 - start_beats.0);
                ratio = ratio.clamp(0.0, 1.0);

                Element::new(cx).background_color(Color::red()).left(Percentage(ratio as f32 * 100.0)).width(Pixels(1.0)).display(if should_display {Display::Flex} else {Display::None});
            });
        });
    });

    // .on_geo_changed(cx, |cx, _| {
    //     // This is a hack until style binding is working
    //     let parent_width = cx.cache.get_width(cx.current);
    //     cx.emit(TrackViewEvent::SetParentWidth(parent_width));

    // });
}

// TODO - Move this to ui state?
#[derive(Debug, Clone, Data, Lens)]
pub struct TracksViewState {
    pub start_time: MusicalTime,
    pub end_time: MusicalTime,
}

impl Model for TracksViewState {

}

#[derive(Debug)]
pub enum TrackViewEvent {

}