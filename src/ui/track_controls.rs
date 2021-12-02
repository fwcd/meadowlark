use vizia::*;

use crate::state::{
    ui_state::{TimelineTrackUiState, UiState},
    AppEvent, StateSystem,
};

pub struct TrackControlsView {
    resizing: bool,
}

impl TrackControlsView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self { resizing: false }
            .build2(cx, move |cx| {
                // Bar Label
                Label::new(cx, "BAR")
                    .height(Pixels(20.0))
                    .width(Stretch(1.0))
                    .child_left(Stretch(0.0))
                    .child_right(Pixels(5.0));

                // Loop Label
                Label::new(cx, "LOOP")
                    .height(Pixels(20.0))
                    .width(Stretch(1.0))
                    .child_left(Stretch(0.0))
                    .child_right(Pixels(5.0))
                    .bottom(Pixels(2.0));

                // Track Controls
                List::new(
                    cx,
                    StateSystem::ui_state.then(UiState::timeline_tracks),
                    |cx, track_data| {
                        //track_controls(cx, track_data);
                        TrackControls::new(cx, track_data.index(), track_data);
                    },
                )
                .row_between(Pixels(2.0))
                .height(Auto);

                // Temporary add track button
                // Button::new(cx, |cx| {println!("Add Track")}, |cx|{
                //     Label::new(cx, "Add Track")
                //         .child_space(Stretch(1.0))
                //         .height(Pixels(30.0))
                //         .width(Stretch(1.0));
                // }).height(Pixels(30.0)).width(Stretch(1.0));
            })
            .width(Pixels(200.0))
    }
}

impl View for TrackControlsView {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if cx.mouse.left.pos_down.0
                        >= cx.cache.get_posx(cx.current) + cx.cache.get_width(cx.current) - 5.0
                        && cx.mouse.left.pos_down.0
                            < cx.cache.get_posx(cx.current) + cx.cache.get_width(cx.current)
                    {
                        self.resizing = true;
                        cx.captured = cx.current;
                        cx.emit(WindowEvent::SetCursor(CursorIcon::EResize));
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    cx.captured = Entity::null();
                    self.resizing = false;

                    // Bit of a hack
                    let cursor =
                        cx.style.borrow().cursor.get(cx.hovered).cloned().unwrap_or_default();
                    cx.emit(WindowEvent::SetCursor(cursor));
                }

                WindowEvent::MouseMove(x, _) => {
                    //if event.target == cx.current {
                    if self.resizing {
                        cx.emit(WindowEvent::SetCursor(CursorIcon::EResize));
                        let mut right = *x - cx.cache.get_posx(cx.current);
                        // Use min/max _width instead
                        right = right.clamp(100.0, 300.0);

                        cx.style
                            .borrow_mut()
                            .width
                            .insert(cx.current, Pixels(right - cx.cache.get_posx(cx.current)));
                        cx.style.borrow_mut().needs_relayout = true;
                        cx.style.borrow_mut().needs_redraw = true;
                    } else {
                        if *x
                            >= cx.cache.get_posx(cx.current) + cx.cache.get_width(cx.current) - 5.0
                            && *x < cx.cache.get_posx(cx.current) + cx.cache.get_width(cx.current)
                        {
                            cx.emit(WindowEvent::SetCursor(CursorIcon::EResize));
                        }
                        // else {
                        //     cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                        // }
                    }

                    //}
                }

                _ => {}
            }
        }
    }
}

pub struct TrackControls {
    track_id: usize,
    resizing: bool,
}

impl TrackControls {
    pub fn new<D>(cx: &mut Context, track_id: usize, track_data: D) -> Handle<Self>
    where
        D: 'static + DataHandle<Data = TimelineTrackUiState>,
    {
        Self { resizing: false, track_id }
            .build2(cx, move |cx| {
                HStack::new(cx, move |cx| {
                    // Track Controls
                    HStack::new(cx, move |cx| {
                        // Track color
                        Element::new(cx)
                            .width(Pixels(10.0))
                            .background_color(Color::rgb(254, 64, 64));
                        VStack::new(cx, move |cx| {
                            HStack::new(cx, move |cx| {
                                let track_data = track_data.get(cx).clone();

                                Label::new(cx, &track_data.name);
                                // Record Button
                                Button::new(cx, |_| {}, |_| {})
                                    .width(Pixels(30.0))
                                    .height(Pixels(30.0));
                                // Solo Button
                                Button::new(cx, |_| {}, |_| {})
                                    .width(Pixels(30.0))
                                    .height(Pixels(30.0));
                                // Mute Button
                                Button::new(cx, |_| {}, |_| {})
                                    .width(Pixels(30.0))
                                    .height(Pixels(30.0));
                            });
                        })
                        .background_color(Color::rgb(179, 172, 174));
                    });
                    // Clips
                })
                .height(Pixels(track_data.get(cx).height));
            })
            .height(Auto)
    }
}

impl View for TrackControls {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if cx.mouse.left.pos_down.1
                        >= cx.cache.get_posy(cx.current) + cx.cache.get_height(cx.current) - 5.0
                        && cx.mouse.left.pos_down.1
                            < cx.cache.get_posy(cx.current) + cx.cache.get_height(cx.current)
                    {
                        self.resizing = true;
                        cx.captured = cx.current;
                        cx.emit(WindowEvent::SetCursor(CursorIcon::NResize));
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    cx.captured = Entity::null();
                    self.resizing = false;

                    // Bit of a hack
                    let cursor =
                        cx.style.borrow().cursor.get(cx.hovered).cloned().unwrap_or_default();
                    cx.emit(WindowEvent::SetCursor(cursor));
                }

                WindowEvent::MouseMove(_, y) => {
                    if self.resizing {
                        cx.emit(WindowEvent::SetCursor(CursorIcon::NResize));
                        let mut bottom = *y - cx.cache.get_posy(cx.current);
                        // Use min/max _width instead
                        bottom = bottom.clamp(100.0, 300.0);
                        //cx.style.borrow_mut().height.insert(cx.current, Pixels(right - cx.cache.get_posx(cx.current)));
                        //cx.style.borrow_mut().needs_relayout = true;
                        //cx.style.borrow_mut().needs_redraw = true;
                        cx.emit(AppEvent::SetTrackHeight(self.track_id, bottom));
                    } else {
                        //println!("y: {} {}", y, cx.cache.get_posy(cx.current) + cx.cache.get_height(cx.current));
                        if *y
                            >= cx.cache.get_posy(cx.current) + cx.cache.get_height(cx.current) - 5.0
                            && *y < cx.cache.get_posy(cx.current) + cx.cache.get_height(cx.current)
                        {
                            cx.emit(WindowEvent::SetCursor(CursorIcon::NResize));
                        }
                        // else {
                        //     cx.emit(WindowEvent::SetCursor(CursorIcon::Default));
                        // }
                    }
                }

                _ => {}
            }
        }
    }
}
