
use rusty_daw_core::MusicalTime;
use vizia::*;

use crate::state::AppEvent;

use super::tracks_view::TracksViewState;


pub struct Clip {
    track_id: usize,
    clip_id: usize,
    clip_start: MusicalTime,
}

impl Clip {
    pub fn new(cx: &mut Context, track_id: usize, clip_id: usize, clip_name: String, clip_start: MusicalTime) -> Handle<Self> {
    
        
        Self {
            clip_start,
            clip_id,
            track_id,
        }.build2(cx, move |cx|{

            if cx.data::<ClipData>().is_none() {
                // Create some internal slider data (not exposed to the user)
                ClipData {
                    dragging: false,
                    start_time: clip_start,
                }.build(cx);
            } else {
                // FIX ME
                let clip_data = cx.data::<ClipData>().unwrap();
                ClipData {
                    dragging: clip_data.dragging,
                    start_time: clip_start,
                }.build(cx);
            }


            Label::new(cx, &clip_name)
                .height(Pixels(20.0))
                .width(Stretch(1.0))
                .background_color(Color::rgb(254, 64, 64))
                .on_press(cx, |cx|{
                    cx.emit(ClipEvent::SetDragging(true));
                });
            Element::new(cx).background_color(Color::rgba(242, 77, 66, 15));
        })
    }
}

impl View for Clip {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseMove(x, _) => {
                    if let Some(clip_data) = cx.data::<ClipData>() {
                        if clip_data.dragging {
                            
                            if let Some(tracks_view_state) = cx.data::<TracksViewState>() {
                                let mut musical_pos = tracks_view_state.delta_to_musical(*x - cx.mouse.left.pos_down.0);
                                // Snapping
                                musical_pos = MusicalTime::new(musical_pos.0.round());

                                //println!("MP: {:?}", clip_data.start_time + musical_pos);
                                cx.emit(AppEvent::SetClipStart(self.track_id, self.clip_id, self.clip_start + musical_pos));

                            }
                            cx.captured = cx.current;
                        }
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    cx.emit(ClipEvent::SetDragging(false));
                    self.clip_start = cx.data::<ClipData>().unwrap().start_time;
                    cx.captured = Entity::null();
                }

                _=> {}
            }
        }
    }
}


#[derive(Debug, Clone, Data, Lens)]
pub struct ClipData {
    dragging: bool,
    // Start time when the clip is pressed
    start_time: MusicalTime,
}

#[derive(Debug)]
pub enum ClipEvent {
    SetDragging(bool),
}

impl Model for ClipData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(clip_event) = event.message.downcast() {
            match clip_event {
                ClipEvent::SetDragging(val) => {
                    self.dragging = *val;
                }
            }
        }
    }
}