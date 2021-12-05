use rusty_daw_core::MusicalTime;
use vizia::*;

use crate::state::{
    ui_state::{LoopUiState, TimelineSelectionUiState, TimelineTransportUiState, UiState},
    StateSystem,
};

use super::{track, LoopRegion, TimelineGrid, TrackControlsView};

pub fn timeline_view(cx: &mut Context) {
    TimelineSelectionUiState {
        track_start: 0,
        track_end: 0,
        select_start: MusicalTime::new(0.0),
        select_end: MusicalTime::new(1.0),

        hovered_track: 0,
    }
    .build(cx);

    HStack::new(cx, |cx| {
        // TODO - Make this resizable
        TrackControlsView::new(cx).background_color(Color::rgb(42, 37, 39));

        if cx.data::<TimelineViewState>().is_none() {
            // Create some internal slider data (not exposed to the user)
            TimelineViewState {
                start_time: MusicalTime::new(0.into()),
                end_time: MusicalTime::new(15.into()),
                timeline_start: MusicalTime::new(0.into()),
                timeline_end: MusicalTime::new(30.into()),
                width: 0.0,
                posx: 0.0,
            }
            .build(cx);
        }

        ZStack::new(cx, |cx| {
            Binding::new(cx, TimelineViewState::root, |cx, track_view_state| {
                let start_beats = track_view_state.get(cx).start_time;
                let end_beats = track_view_state.get(cx).end_time;
                let timeline_start = track_view_state.get(cx).timeline_start;
                let timeline_end = track_view_state.get(cx).timeline_end;
                let timeline_width = track_view_state.get(cx).width;

                Binding::new(
                    cx,
                    StateSystem::ui_state
                        .then(UiState::timeline_transport)
                        .then(TimelineTransportUiState::playhead),
                    move |cx, playhead| {
                        let timeline_beats = end_beats - start_beats;

                        // Grid lines
                        TimelineGrid::new(cx).z_order(1).hoverable(false);

                        VStack::new(cx, move |cx| {
                            // Bars
                            Element::new(cx).height(Pixels(20.0));

                            // Loop Bar
                            ZStack::new(cx, move |cx| {
                                Binding::new(
                                    cx,
                                    StateSystem::ui_state
                                        .then(UiState::timeline_transport)
                                        .then(TimelineTransportUiState::loop_state),
                                    move |cx, loop_state| {
                                        let loop_state = loop_state.get(cx);
                                        match loop_state {
                                            LoopUiState::Active { loop_start, loop_end } => {
                                                let loop_start_pos = (loop_start.0 - start_beats.0)
                                                    / (end_beats.0 - start_beats.0);
                                                let loop_end_pos = (loop_end.0 - start_beats.0)
                                                    / (end_beats.0 - start_beats.0);
                                                //println!("loop_start: {:?} loop_end: {:?} start_beats: {:?} end_beats: {:?}", loop_start, loop_end, start_beats, end_beats);
                                                let should_display = *loop_start >= start_beats
                                                    || *loop_end >= start_beats;
                                                LoopRegion::new(cx)
                                                    .display(if should_display {
                                                        Display::Flex
                                                    } else {
                                                        Display::None
                                                    })
                                                    .background_color(Color::rgba(
                                                        50, 100, 255, 120,
                                                    ))
                                                    .width(Stretch(1.0))
                                                    .left(Percentage(loop_start_pos as f32 * 100.0))
                                                    .right(Percentage(
                                                        (1.0 - loop_end_pos as f32) * 100.0,
                                                    ));
                                            }

                                            LoopUiState::Inactive => {
                                                Element::new(cx).display(Display::None);
                                            }
                                        }
                                    },
                                );
                            })
                            .height(Pixels(20.0))
                            .background_color(Color::rgb(68, 60, 60))
                            .bottom(Pixels(2.0));

                            // Tracks
                            List::new(
                                cx,
                                StateSystem::ui_state.then(UiState::timeline_tracks),
                                |cx, track_data| {
                                    track(cx, track_data.index(), track_data);
                                },
                            )
                            .row_between(Pixels(2.0));

                            // Scrollbar
                            //HStack::new(cx, move |cx|{
                            //Button::new(cx, |_|{}, |_|{}).width(Pixels(15.0)).height(Pixels(15.0));
                            ZStack::new(cx, move |cx| {
                                let width_ratio = timeline_beats / (timeline_end - timeline_start);
                                let start_ratio = (start_beats - timeline_start)
                                    / (timeline_end - timeline_start);
                                //Element::new(cx)
                                ScrollBar::new(cx)
                                    .left(Pixels(start_ratio.0 as f32 * timeline_width))
                                    .width(Pixels(width_ratio.0 as f32 * timeline_width))
                                    .background_color(Color::rgb(126, 118, 119));
                            })
                            .child_space(Pixels(1.0))
                            .background_color(Color::rgb(36, 36, 36))
                            .height(Pixels(15.0))
                            .z_order(5);
                            //Button::new(cx, |_|{}, |_|{}).width(Pixels(15.0)).height(Pixels(15.0));
                            //});
                        });

                        let current_beats = playhead.get(cx);

                        let should_display =
                            current_beats.0 >= start_beats.0 && current_beats.0 <= end_beats.0;

                        let mut ratio =
                            (current_beats.0 - start_beats.0) / (end_beats.0 - start_beats.0);
                        ratio = ratio.clamp(0.0, 1.0);

                        // Playhead
                        Element::new(cx)
                            .background_color(Color::rgb(170, 161, 164))
                            .left(Percentage(ratio as f32 * 100.0))
                            .width(Pixels(1.0))
                            .display(if should_display { Display::Flex } else { Display::None })
                            .z_order(4);
                    },
                );
            });
        })
        .background_color(Color::rgb(42, 37, 39))
        .overflow(Overflow::Hidden)
        .on_move(cx, |cx, x, y| {
            if x >= cx.cache.get_posx(cx.current) + cx.cache.get_width(cx.current) - 10.0
                && x <= cx.cache.get_posx(cx.current) + cx.cache.get_width(cx.current)
            {
                // Moves the timeline view forwards when the mouse is on the right edge
                // TODO - sync with tick rate
                //cx.emit(TimelineViewEvent::ShiftForwards(MusicalTime::new(1.0)));
            }

            if x > cx.cache.get_posx(cx.current) && x < cx.cache.get_posx(cx.current) + 10.0 {
                // Moves the timeline view backwards when the mouse is on the left edge
                // TODO - sync with tick rate
                //cx.emit(TimelineViewEvent::ShiftBackwards(MusicalTime::new(1.0)));
            }
        })
        .on_geo_changed(cx, |cx, geo| {
            if geo.contains(GeometryChanged::WIDTH_CHANGED) {
                cx.emit(TimelineViewEvent::SetWidth(cx.cache.get_width(cx.current)));
            }

            if geo.contains(GeometryChanged::POSX_CHANGED) {
                cx.emit(TimelineViewEvent::SetPosx(cx.cache.get_posx(cx.current)));
            }
        });
    });
}

// TODO - Move this to ui state?
#[derive(Debug, Clone, Data, Lens)]
pub struct TimelineViewState {
    pub start_time: MusicalTime,
    pub end_time: MusicalTime,
    pub timeline_start: MusicalTime,
    pub timeline_end: MusicalTime,
    pub width: f32,
    pub posx: f32,
}

impl TimelineViewState {
    // Converts absolute cursor position into musical time
    pub fn cursor_to_musical(&self, cursorx: f32) -> MusicalTime {
        let beats = self.start_time.0
            + ((cursorx - self.posx) / self.width) as f64 * (self.end_time.0 - self.start_time.0);
        MusicalTime::new(beats.into())
    }

    // Converts delta cursor movement into musical time
    pub fn delta_to_musical(&self, cursorx: f32) -> MusicalTime {
        let beats = (cursorx / self.width) as f64 * (self.end_time.0 - self.start_time.0);
        MusicalTime::new(beats.into())
    }

    pub fn delta_to_musical2(&self, cursorx: f32) -> MusicalTime {
        let beats = (cursorx / self.width) as f64 * (self.timeline_end.0 - self.timeline_start.0);
        MusicalTime::new(beats.into())
    }
}

impl Model for TimelineViewState {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(track_view_event) = event.message.downcast() {
            match track_view_event {
                TimelineViewEvent::SetWidth(val) => {
                    self.width = *val;
                }

                TimelineViewEvent::SetPosx(val) => {
                    self.posx = *val;
                }

                TimelineViewEvent::ShiftForwards(time_shift) => {
                    self.start_time += *time_shift;
                    self.end_time += *time_shift;
                }

                TimelineViewEvent::ShiftBackwards(time_shift) => {
                    if self.start_time >= *time_shift {
                        self.start_time -= *time_shift;
                        self.end_time -= *time_shift;
                    }
                }

                TimelineViewEvent::SetStartTime(start_time) => {
                    if self.end_time - *start_time >= MusicalTime::new(0.2) {
                        self.start_time = MusicalTime::new(start_time.0.max(self.timeline_start.0));
                    }
                }

                TimelineViewEvent::SetEndTime(end_time) => {
                    if *end_time - self.start_time >= MusicalTime::new(0.2) {
                        self.end_time = MusicalTime::new(end_time.0.min(self.timeline_end.0));
                    }
                }

                TimelineViewEvent::SetTimelineStart(timeline_start) => {
                    self.timeline_start = *timeline_start;
                }

                TimelineViewEvent::SetTimelineEnd(timeline_end) => {
                    self.timeline_end = *timeline_end;
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TimelineViewEvent {
    // Set the width of the tracks view
    SetWidth(f32),
    // Set the posx of the tracks view
    SetPosx(f32),

    ShiftForwards(MusicalTime),

    ShiftBackwards(MusicalTime),

    SetStartTime(MusicalTime),

    SetEndTime(MusicalTime),

    SetTimelineStart(MusicalTime),

    SetTimelineEnd(MusicalTime),
}

pub struct ScrollBar {
    drag_start: bool,
    drag_end: bool,
    dragging: bool,

    // When clicked
    timeline_start: MusicalTime,
    timeline_end: MusicalTime,
    start_time: MusicalTime,
    end_time: MusicalTime,

    left_edge: f32,
}

impl ScrollBar {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            drag_start: false,
            drag_end: false,
            dragging: false,

            timeline_start: MusicalTime::new(0.0),
            timeline_end: MusicalTime::new(0.0),
            start_time: MusicalTime::new(0.0),
            end_time: MusicalTime::new(0.0),
            

            left_edge: 0.0,
        }
        .build2(cx, |cx| {})
    }
}

impl View for ScrollBar {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        let local_click_pos =
                            cx.mouse.left.pos_down.0 - cx.cache.get_posx(cx.current);
                        if local_click_pos >= 0.0 && local_click_pos <= 5.0 {
                            self.drag_start = true;
                            self.left_edge = cx.cache.get_posx(cx.current);
                        } else if local_click_pos >= cx.cache.get_width(cx.current) - 5.0
                            && local_click_pos <= cx.cache.get_width(cx.current)
                        {
                            self.drag_end = true;
                        } else {
                            self.dragging = true;
                            self.left_edge = cx.cache.get_posx(cx.current);
                        }
                        cx.captured = cx.current;

                        if let Some(timeline_view_state) = cx.data::<TimelineViewState>() {
                            self.timeline_start = timeline_view_state.timeline_start;
                            self.timeline_end = timeline_view_state.timeline_end;
                            self.start_time = timeline_view_state.start_time;
                            self.end_time = timeline_view_state.end_time;
                            println!("This: {:?}", self.start_time);
                        }
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        self.drag_start = false;
                        self.drag_end = false;
                        self.dragging = false;
                        cx.captured = Entity::null();
                        if cx.hovered != cx.current {
                            cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                        }
                    }
                }

                WindowEvent::MouseMove(x, y) => {
                    let local_mouse_pos = *x - cx.cache.get_posx(cx.current);

                    if self.drag_start || self.drag_end {
                        cx.emit(WindowEvent::SetCursor(CursorIcon::EwResize));
                    } else {
                        if local_mouse_pos >= 0.0 && local_mouse_pos <= 5.0
                            || local_mouse_pos >= cx.cache.get_width(cx.current) - 5.0
                                && local_mouse_pos <= cx.cache.get_width(cx.current)
                        {
                            cx.emit(WindowEvent::SetCursor(CursorIcon::EwResize));
                        } else {
                            cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                        }
                    }
                    if let Some(timeline_view_state) = cx.data::<TimelineViewState>() {
                        let timeline_width = timeline_view_state.width;
                        let timeline_posx = timeline_view_state.posx;
                        let start_time = timeline_view_state.start_time;
                        let end_time = timeline_view_state.end_time;

                        if self.dragging {
                            //let mut delta = (*x - timeline_posx) / timeline_width;
                            let delta = (*x - cx.mouse.left.pos_down.0) / timeline_width;
                            let start_time =
                                MusicalTime::new(delta.into()) * (self.timeline_end - self.timeline_start);
                            //println!("Start Time: {:?}", self.start_time + start_time);
                            let musical = timeline_view_state.delta_to_musical2(*x - cx.mouse.left.pos_down.0);
                            //println!("Start Time: {:?}", self.timeline_start);
                            //println!("New Start: {:?}", musical);
                            if self.start_time.0 + musical.0 <= self.timeline_start.0 {
                                cx.emit(TimelineViewEvent::SetStartTime(self.timeline_start));
                                cx.emit(TimelineViewEvent::SetEndTime(self.timeline_start + (self.end_time - self.start_time)));
                            } else if self.end_time.0 + musical.0 >= self.timeline_end.0 {
                                cx.emit(TimelineViewEvent::SetEndTime(self.timeline_end));
                                cx.emit(TimelineViewEvent::SetStartTime(self.timeline_end - (self.end_time - self.start_time)));
                            } else {
                                cx.emit(TimelineViewEvent::SetStartTime(self.start_time + musical));
                                cx.emit(TimelineViewEvent::SetEndTime(self.end_time + musical));
                            }
                        }

                        if self.drag_end {
                            let mut delta = (*x - timeline_posx) / timeline_width;
                            delta = delta.clamp(0.0, 1.0);
                            let end_time =
                                MusicalTime::new(delta.into()) * (self.timeline_end - self.timeline_start);
                            cx.emit(TimelineViewEvent::SetEndTime(end_time));
                        }

                        if self.drag_start {
                            let mut delta = (*x - timeline_posx) / timeline_width;
                            delta = delta.clamp(0.0, 1.0);
                            let start_time =
                                MusicalTime::new(delta.into()) * (self.timeline_end - self.timeline_start);
                            cx.emit(TimelineViewEvent::SetStartTime(start_time));
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
