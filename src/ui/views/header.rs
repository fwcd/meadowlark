use vizia::*;

use super::control_bars::*;

#[derive(Default)]
pub struct Header {}

impl Header {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self{}.build(cx).class("header")
    }
}

impl View for Header {
    fn body(&mut self, cx: &mut Context) {
        TempoControlBar::new(cx);
        Element::new(cx).class("spacer");
        TransportControlBar::new(cx);
    }
}
