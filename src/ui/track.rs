use vizia::*;

use crate::state::{
    ui_state::{
        TempoMapUiState, TimelineSelectionEvent, TimelineSelectionUiState, TimelineTrackUiState,
        UiState,
    },
    StateSystem,
};

use super::{tracks_view::TracksViewState, Clip};

pub fn track<D>(cx: &mut Context, track_id: usize, track_data: D)
where
    D: 'static + DataHandle<Data = TimelineTrackUiState>,
{
    // This ZStack isn't strictly necessary but the bindings mess with the list so this is a temporary fix
    ZStack::new(cx, move |cx| {
        Binding::new(
            cx,
            StateSystem::ui_state.then(UiState::tempo_map).then(TempoMapUiState::bpm),
            move |cx, bpm| {
                Binding::new(cx, TracksViewState::root, move |cx, track_view_state| {
                    let start_beats = track_view_state.get(cx).start_time;
                    let end_beats = track_view_state.get(cx).end_time;
                    let timeline_beats = end_beats - start_beats;
                    HStack::new(cx, move |cx| {
                        let clip_data = track_data.get(cx).audio_clips.clone();

                        if cx.current.child_iter(&cx.tree.clone()).count() != clip_data.len() {
                            for child in cx.current.child_iter(&cx.tree.clone()) {
                                cx.remove(child);
                            }

                            cx.style.borrow_mut().needs_relayout = true;
                            cx.style.borrow_mut().needs_redraw = true;
                        }

                        for (clip_id, clip) in clip_data.iter().enumerate() {
                            let clip_start = clip.timeline_start.clone();
                            let clip_name = clip.name.clone();
                            let duration = clip.duration.to_musical(*bpm.get(cx) as f64);
                            let clip_end = clip_start + duration;

                            let clip_start_pos =
                                (clip_start.0 - start_beats.0).max(0.0) / timeline_beats.0;
                            let clip_end_pos =
                                (clip_end.0 - start_beats.0).max(0.0) / timeline_beats.0;

                            let should_display =
                                clip_start >= start_beats || clip_end >= start_beats;

                            Clip::new(cx, track_id, clip_id, clip_name, clip_start, clip_end)
                                //.display(if should_display {Display::Flex} else {Display::None})
                                .left(Percentage(clip_start_pos as f32 * 100.0))
                                .right(Percentage((1.0 - clip_end_pos as f32) * 100.0))
                                .width(Stretch(1.0))
                                .z_order(2);
                        }
                    })
                    .background_color(Color::rgb(68, 60, 60));

                    // Selection - TODO
                    Binding::new(cx, TimelineSelectionUiState::root, move |cx, selection| {
                        let select_start = selection.get(cx).select_start;
                        let select_end = selection.get(cx).select_end;
                        let track_start = selection.get(cx).track_start;
                        let track_end = selection.get(cx).track_end;
                        let should_display = track_id >= track_start && track_id <= track_end;
                        Element::new(cx)
                            .display(if should_display { Display::Flex } else { Display::None })
                            .background_color(Color::rgba(50, 200, 250, 100))
                            .width(Stretch(1.0))
                            .left(Percentage(100.0 * (select_start.0 / timeline_beats.0) as f32))
                            .right(Percentage(
                                100.0 * (1.0 - (select_end.0 / timeline_beats.0) as f32),
                            ));
                    });
                });
            },
        );
    })
    .height(Pixels(track_data.get(cx).height))
    .background_color(Color::rgb(68, 60, 60))
    .on_over(cx, move |cx| {
        cx.emit(TimelineSelectionEvent::SetHoveredTrack(track_id));
    });
}
