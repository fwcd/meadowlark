use rusty_daw_core::MusicalTime;
use vizia::*;

use crate::state::AppEvent;

use super::timeline_view::TimelineViewState;

pub struct LoopRegion {
    drag_start: bool,
    drag_end: bool,
}

impl LoopRegion {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { drag_start: false, drag_end: false }.build2(cx, |_| {})
    }
}

impl View for LoopRegion {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseLeave => {
                    if !self.drag_start && !self.drag_end {
                        cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                    }
                }

                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        let local_click_pos =
                            cx.mouse.left.pos_down.0 - cx.cache.get_posx(cx.current);
                        if local_click_pos >= 0.0 && local_click_pos <= 5.0 {
                            self.drag_start = true;
                        }

                        if local_click_pos >= cx.cache.get_width(cx.current) - 5.0
                            && local_click_pos <= cx.cache.get_width(cx.current)
                        {
                            self.drag_end = true;
                        }

                        cx.captured = cx.current;
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        self.drag_start = false;
                        self.drag_end = false;
                        cx.captured = Entity::null();
                        if cx.hovered != cx.current {
                            cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                        }
                    }
                }

                WindowEvent::MouseMove(x, _) => {
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
                        let mut musical_pos = timeline_view_state.cursor_to_musical(*x);
                        // Snapping
                        musical_pos = MusicalTime::new(musical_pos.0.round());
                        if self.drag_end {
                            cx.emit(AppEvent::SetLoopEnd(musical_pos));
                        }

                        if self.drag_start {
                            cx.emit(AppEvent::SetLoopStart(musical_pos));
                        }
                    }
                }

                _ => {}
            }
        }
    }
}
