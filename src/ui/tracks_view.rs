


use rusty_daw_core::MusicalTime;
use vizia::*;

use crate::state::{StateSystem, ui_state::{LoopUiState, TimelineTransportUiState, UiState}};

use super::{LoopRegion, track, track_controls};

pub fn tracks_view(cx: &mut Context) {

    HStack::new(cx, |cx|{

        // TODO - Make this resizable
        VStack::new(cx, |cx|{
            // Loop Label
            Label::new(cx, "LOOP")
                .height(Pixels(20.0))
                .width(Stretch(1.0))
                .child_left(Stretch(0.0))
                .child_right(Pixels(5.0))
                .bottom(Pixels(2.0));

            // Track Controls
            List::new(cx, StateSystem::ui_state.then(UiState::timeline_tracks), |cx, track_data|{
                track_controls(cx, track_data);
            }).row_between(Pixels(2.0)); 
        }).width(Pixels(200.0)).background_color(Color::rgb(42, 37, 39));


        if cx.data::<TracksViewState>().is_none() {
            // Create some internal slider data (not exposed to the user)
            TracksViewState {
                start_time: MusicalTime::new(0.into()),
                end_time: MusicalTime::new(20.into()),
                width: 0.0,
                posx: 0.0,
            }.build(cx);
        }

        ZStack::new(cx, |cx|{

            Binding::new(cx, TracksViewState::root, |cx, track_view_state|{
                let start_beats = track_view_state.get(cx).start_time;
                let end_beats = track_view_state.get(cx).end_time;
                Binding::new(cx, StateSystem::ui_state.then(UiState::timeline_transport).then(TimelineTransportUiState::playhead), move |cx, playhead|{
                    
                    // Grid lines
                    for i in 0..20 {
                        let ratio = (i as f64 - start_beats.0) / (end_beats.0 - start_beats.0);
                        Element::new(cx).width(Pixels(1.0)).left(Percentage(ratio as f32 * 100.0)).background_color(Color::rgb(36, 36, 36)).z_order(1);
                    }

                    VStack::new(cx, move |cx|{
                        
                        // Loop Bar
                        ZStack::new(cx, move |cx|{
                            Binding::new(cx, StateSystem::ui_state.then(UiState::timeline_transport).then(TimelineTransportUiState::loop_state), move |cx, loop_state|{
                                let loop_state = loop_state.get(cx);
                                match loop_state {
                                    LoopUiState::Active{
                                        loop_start,
                                        loop_end,
                                    } => {
                                        let loop_start_pos = loop_start.0 / (end_beats.0 - start_beats.0);
                                        let loop_end_pos = loop_end.0 / (end_beats.0 - start_beats.0);
                                        //Element::new(cx).background_color(Color::red()).width(Stretch(1.0)).left(Percentage(loop_start_pos as f32 * 100.0)).right(Percentage((1.0 - loop_end_pos as f32) * 100.0));
                                        LoopRegion::new(cx).background_color(Color::rgba(50, 100, 255, 120)).width(Stretch(1.0)).left(Percentage(loop_start_pos as f32 * 100.0)).right(Percentage((1.0 - loop_end_pos as f32) * 100.0));
                                    }

                                    LoopUiState::Inactive => {
                                        Element::new(cx).display(Display::None);
                                    }

                                }
                            });

                        }).height(Pixels(20.0)).background_color(Color::rgb(68, 60, 60)).bottom(Pixels(2.0));

                        // Tracks
                        List::new(cx, StateSystem::ui_state.then(UiState::timeline_tracks), |cx, track_data|{
                            track(cx, track_data);
                        }).row_between(Pixels(2.0));
                    });
                    


                    let current_beats = playhead.get(cx);
                    
                    let should_display = current_beats.0 >= start_beats.0 && current_beats.0 <= end_beats.0;

                    let mut ratio = (current_beats.0 - start_beats.0) / (end_beats.0 - start_beats.0);
                    ratio = ratio.clamp(0.0, 1.0);

                    // Playhead
                    Element::new(cx).background_color(Color::rgb(170, 161, 164)).left(Percentage(ratio as f32 * 100.0)).width(Pixels(1.0)).display(if should_display {Display::Flex} else {Display::None}).z_order(4);
                });
            });
        }).background_color(Color::rgb(42,37,39))
        .on_geo_changed(cx, |cx, geo|{
            if geo.contains(GeometryChanged::WIDTH_CHANGED) {
                cx.emit(TracksViewEvent::SetWidth(cx.cache.get_width(cx.current)));
            }

            if geo.contains(GeometryChanged::POSX_CHANGED) {
                cx.emit(TracksViewEvent::SetPosx(cx.cache.get_posx(cx.current)));
            }
        });        
    });
}

// TODO - Move this to ui state?
#[derive(Debug, Clone, Data, Lens)]
pub struct TracksViewState {
    pub start_time: MusicalTime,
    pub end_time: MusicalTime,
    pub width: f32,
    pub posx: f32,
}

impl TracksViewState {
    pub fn cursor_to_musical(&self, cursorx: f32) -> MusicalTime {
        let beats = ((cursorx - self.posx) / self.width) * (self.end_time.0 - self.start_time.0) as f32;
        MusicalTime::new(beats.into())
    }
}

impl Model for TracksViewState {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(track_view_event) = event.message.downcast() {
            match track_view_event {
                TracksViewEvent::SetWidth(val) => {
                    self.width = *val;
                }

                TracksViewEvent::SetPosx(val) => {
                    self.posx = *val;
                }
            }
        }
        
    }
}

#[derive(Debug)]
pub enum TracksViewEvent {
    // Set the width of the tracks view
    SetWidth(f32),
    // Set the posx of the tracks view
    SetPosx(f32),
}