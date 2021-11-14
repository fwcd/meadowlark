use vizia::*;

use crate::state::{BoundGuiState};

/// Widget for the TEMPO control bar
#[derive(Default)]
pub struct TempoControlBar {}

impl TempoControlBar {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self{}.build(cx).class("control_bar")
    }
}

impl View for TempoControlBar {
    fn body(&mut self, cx: &mut Context) {
        //let controls = ControlBar::new("TEMPO").build(state, entity, |builder| builder);

        Label::new(cx, "TEMPO");

        HStack::new(cx, |cx|{
            Binding::new(cx, BoundGuiState::bpm, |cx, bpm|{
                let bpm = bpm.get(cx);
                Label::new(cx, &format!("BPM: {}", bpm.to_string()));
            });
        });

        // Textbox::new("130")
        //     .on_submit(|data, state, textbox| {
        //         if let Ok(bpm) = data.text.parse::<f64>() {
        //             textbox.emit(state, TempoEvent::SetBPM(bpm).to_state_event());
        //         } else {
        //             // TODO - need better error handling/ fallback here
        //             data.text = "130".to_string();
        //         }
        //     })
        //     .bind(BoundGuiState::bpm, |value| value.to_string())
        //     .build(state, controls, |builder| builder.set_name("tempo"));

        // Button::with_label("TAP").build(state, controls, |builder| builder);
        // Button::with_label("4/4")
        //     .build(state, controls, |builder| builder.set_name("time signature"));

        // Dropdown::new("GROOVE").build(state, controls, |builder| builder);

        // Button::with_label("GROOVE").build(state, controls, |builder|
        //     builder
        //         .set_disabled(true)
        // );

        //entity.class(state, "control_bar").set_name(state, "tempo controls")
    }
}

// #[derive(Debug, Clone, PartialEq)]
// enum TempoTapEvent {
//     Tapped(f32),
// }

// /// Widget for the TAP tempo button
// #[derive(Default)]
// pub struct TempoTapButton {}

// impl Widget for TempoTapButton {
//     type Ret = Entity;
//     type Data = ();
//     fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
//         Button::with_label("TAP")
//             //.on_press(callback)
//             .build(state, entity, |builder| builder);

//         entity
//     }

//     fn on_event(&mut self, _state: &mut State, _entity: Entity, _event: &mut Event) {}
// }
