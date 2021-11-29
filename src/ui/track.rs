use vizia::*;

use crate::state::{StateSystem, ui_state::{TempoMapUiState, TimelineTrackUiState, UiState}};

use super::{Clip, tracks_view::TracksViewState};

pub fn track<D>(cx: &mut Context, track_id: usize, track_data: D) 
where D: 'static + DataHandle<Data = TimelineTrackUiState>,
{
    // This ZStack isn't strictly necessary but the bindings mess with the list so this is a temporary fix
    ZStack::new(cx, move |cx|{
        Binding::new(cx, StateSystem::ui_state.then(UiState::tempo_map).then(TempoMapUiState::bpm), move |cx, bpm|{
            Binding::new(cx, TracksViewState::root, move |cx, track_view_state|{
                let start_beats = track_view_state.get(cx).start_time;
                let end_beats = track_view_state.get(cx).end_time;
                HStack::new(cx, move |cx|{
                    let clip_data = track_data.get(cx).audio_clips.clone();
                    for (clip_id, clip) in clip_data.iter().enumerate() {
                        let clip_start = clip.timeline_start.clone();
                        let clip_name = clip.name.clone();
                        let duration = clip.duration.to_musical(*bpm.get(cx) as f64);
                        let mut width_ratio = duration.0 / (end_beats.0 - start_beats.0);
                        width_ratio = width_ratio.clamp(0.0, 1.0);
                        let mut ratio = (clip_start.0 - start_beats.0) / (end_beats.0 - start_beats.0);
                        ratio = ratio.clamp(0.0, 1.0);


                        Clip::new(cx, track_id, clip_id, clip_name, clip_start).left(Percentage(ratio as f32 * 100.0)).width(Percentage(width_ratio as f32 * 100.0)).z_order(2);

                        // VStack::new(cx, move |cx|{
                        //     Label::new(cx, &clip_name).height(Pixels(20.0)).width(Stretch(1.0)).background_color(Color::rgb(254, 64, 64));
                        //     Element::new(cx).background_color(Color::rgba(242, 77, 66, 15));
                        // }).left(Percentage(ratio as f32 * 100.0)).width(Percentage(width_ratio as f32 * 100.0)).z_order(2);
                        //.width(Percentage(width_ratio as f32 * 100.0));
                    }
                }).height(Pixels(100.0)).background_color(Color::rgb(68, 60, 60));
            });
        });        
    }).height(Pixels(100.0)).background_color(Color::rgb(68, 60, 60));



}