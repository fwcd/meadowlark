use rusty_daw_core::MusicalTime;
use vizia::*;

use crate::state::{AppEvent, ui_state::{TimelineSelectionEvent, TimelineSelectionUiState}};

use super::tracks_view::TracksViewState;

pub struct Clip {
    track_id: usize,
    clip_id: usize,
    clip_start: MusicalTime,
    clip_end: MusicalTime,
    resize_start: bool,
    resize_end: bool,
}

impl Clip {
    pub fn new(
        cx: &mut Context,
        track_id: usize,
        clip_id: usize,
        clip_name: String,
        clip_start: MusicalTime,
        clip_end: MusicalTime,
    ) -> Handle<Self> {
        
        Self { 
            track_id,
            clip_id, 
            clip_start, 
            clip_end,
            resize_start: false,
            resize_end: false,
        }.build2(cx, move |cx| {
            if cx.data::<ClipData>().is_none() {
                // Create some internal slider data (not exposed to the user)
                ClipData { dragging: false, start_time: clip_start, end_time: clip_end }.build(cx);
            } else {
                // FIX ME
                let clip_data = cx.data::<ClipData>().unwrap();
                ClipData { dragging: clip_data.dragging, start_time: clip_start, end_time: clip_end }.build(cx);
            }

            Label::new(cx, &clip_name)
                .height(Pixels(20.0))
                .width(Stretch(1.0))
                .background_color(Color::rgb(254, 64, 64))
                .class("clip_header")
                .on_press(cx, move |cx| {
                    cx.emit(ClipEvent::SetDragging(true));
                    cx.emit(TimelineSelectionEvent::SetSelection(track_id, track_id, clip_start, clip_end));
                });
            Element::new(cx).background_color(Color::rgba(242, 77, 66, 15));
        }).position_type(PositionType::SelfDirected)
    }
}

impl View for Clip {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseMove(x, _) => {
                    if let Some(clip_data) = cx.data::<ClipData>() {

                        let start_time = clip_data.start_time;
                        let end_time = clip_data.end_time;

                        if clip_data.dragging {
                            if let Some(tracks_view_state) = cx.data::<TracksViewState>() {
                                let mut musical_pos = tracks_view_state
                                    .delta_to_musical(*x - cx.mouse.left.pos_down.0);
                                // Snapping
                                musical_pos = MusicalTime::new(musical_pos.0.round());

                                

                                //println!("MP: {:?}", clip_data.start_time + musical_pos);
                                cx.emit(AppEvent::SetClipStart(
                                    self.track_id,
                                    self.clip_id,
                                    self.clip_start + musical_pos,
                                ));
                                
                                cx.emit(TimelineSelectionEvent::SetSelection(self.track_id, self.track_id, start_time, end_time));

                            }
                            cx.captured = cx.current;
                            cx.emit(WindowEvent::SetCursor(CursorIcon::Grabbing));
                        }

                        let local_mouse_pos = *x - cx.cache.get_posx(cx.current);

                        if self.resize_start || self.resize_end {
                            cx.emit(WindowEvent::SetCursor(CursorIcon::EwResize));
                        } else {
                            if local_mouse_pos >= 0.0 && local_mouse_pos <= 5.0
                                || local_mouse_pos >= cx.cache.get_width(cx.current) - 5.0
                                    && local_mouse_pos <= cx.cache.get_width(cx.current)
                            {
                                cx.emit(WindowEvent::SetCursor(CursorIcon::EwResize));
                            } else {
                                //cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                                let cursor = cx.style.borrow().cursor.get(cx.hovered).cloned().unwrap_or_default();
                                cx.emit(WindowEvent::SetCursor(cursor));
                            }
                        }
    
                        if let Some(tracks_view_state) = cx.data::<TracksViewState>() {
                            let mut musical_pos = tracks_view_state.cursor_to_musical(*x);
                            // Snapping
                            musical_pos = MusicalTime::new(musical_pos.0.round());
                            if self.resize_end {
                                cx.emit(AppEvent::SetClipEnd(self.track_id, self.clip_id, musical_pos));
                                cx.emit(TimelineSelectionEvent::SetSelection(self.track_id, self.track_id, start_time, end_time));

                            }
    
                            if self.resize_start {
                                cx.emit(AppEvent::SetClipStart(self.track_id, self.clip_id, musical_pos));
                                cx.emit(AppEvent::SetClipEnd(self.track_id, self.clip_id, self.clip_end));
                                cx.emit(TimelineSelectionEvent::SetSelection(self.track_id, self.track_id, start_time, end_time));

                            }
                        }
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    cx.emit(ClipEvent::SetDragging(false));
                    self.clip_start = cx.data::<ClipData>().unwrap().start_time;
                    //cx.captured = Entity::null();
                    //let cursor =
                        //cx.style.borrow().cursor.get(cx.hovered).cloned().unwrap_or_default();
                    //cx.emit(WindowEvent::SetCursor(cursor));

                    if event.target == cx.current {
                        self.resize_start = false;
                        self.resize_end = false;
                        cx.captured = Entity::null();
                        if cx.hovered != cx.current {
                            //cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                            let cursor = cx.style.borrow().cursor.get(cx.hovered).cloned().unwrap_or_default();
                            cx.emit(WindowEvent::SetCursor(cursor));
                        }
                    }
                }

                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    cx.focused = cx.current;

                    //if event.target == cx.current {
                        let local_click_pos =
                            cx.mouse.left.pos_down.0 - cx.cache.get_posx(cx.current);
                        if local_click_pos >= 0.0 && local_click_pos <= 5.0 {
                            self.resize_start = true;
                            cx.emit(ClipEvent::SetDragging(false));
                        }

                        if local_click_pos >= cx.cache.get_width(cx.current) - 5.0
                            && local_click_pos <= cx.cache.get_width(cx.current)
                        {
                            self.resize_end = true;
                            cx.emit(ClipEvent::SetDragging(false));
                        }

                        cx.captured = cx.current;
                    //}
                }

                // TEMPORARY - Need to move this to a keymap that wraps the timeline
                WindowEvent::KeyDown(code, _) => match code {

                    Code::KeyD => {
                        if cx.modifiers.contains(Modifiers::CTRL) {
                            println!("Duplicate");
                            if let Some(timeline_selection) = cx.data::<TimelineSelectionUiState>() {
                                cx.emit(AppEvent::Duplicate(timeline_selection.track_start, timeline_selection.select_start, timeline_selection.select_end));
                            }
                        }
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Data, Lens)]
pub struct ClipData {
    dragging: bool,
    // Start time when the clip is pressed
    start_time: MusicalTime,

    end_time: MusicalTime,
}

#[derive(Debug)]
pub enum ClipEvent {
    SetDragging(bool),
}

impl Model for ClipData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(clip_event) = event.message.downcast() {
            match clip_event {
                ClipEvent::SetDragging(val) => {
                    self.dragging = *val;
                }
            }
        }
    }
}
