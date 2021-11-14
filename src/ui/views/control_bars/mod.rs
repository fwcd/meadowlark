
use vizia::*;

pub mod tempo;
pub use tempo::*;

pub mod transport;
pub use transport::*;

// pub struct ControlBar {
//     name: String,
// }

// impl ControlBar {
//     pub fn new(name: &str) -> Handle<Self> {
//         Self { name: name.to_string() }
//     }
// }

// impl View for ControlBar {
//     fn body(&mut self, cx: &mut Context) {
//         Label::new(cx, &self.name);

//         let controls = Row::new().build(state, entity, |builder| builder.class("controls"));

//         entity.class(state, "control_bar").set_focusable(state, false);

//         controls
//     }
// }
