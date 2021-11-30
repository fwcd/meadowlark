use vizia::*;

use crate::state::{AppEvent, StateSystem};

// Handles keyboard events and translates them to app events
// TODO - This should probably be part of vizia?

pub struct Keymap {}

impl Keymap {
    pub fn new<F>(cx: &mut Context, builder: F) -> Handle<Self>
    where
        F: 'static + FnOnce(&mut Context),
    {
        let handle = Self {}.build2(cx, |cx| {
            (builder)(cx);
        });

        cx.focused = cx.current;

        handle
    }
}

impl View for Keymap {
    fn event(&mut self, cx: &mut Context, event: &mut vizia::Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::KeyDown(code, key) => match code {
                    Code::Space => {
                        cx.emit(AppEvent::PlayPause);
                    }

                    _ => {}
                },

                _ => {}
            }
        }
    }
}
