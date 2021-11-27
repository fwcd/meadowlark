use vizia::*;

use crate::state::{AppEvent, ProjectSaveState, StateSystem};

mod tempo_controls;
use tempo_controls::tempo_controls;

mod transport_controls;
use transport_controls::transport_controls;

mod tracks_view;
use tracks_view::tracks_view;

const STYLE: &str = r#"
    .divider {
        top: 1s;
        bottom: 1s;
        width: 2px;
        height: 4s;
        background-color: #242424;
    }

    label {
        color: white;
    }
"#;


pub fn run() {
    Application::new(|cx|{
        let project_save_state = Box::new(ProjectSaveState::test());
        let mut state_system = StateSystem::new();
        state_system.load_project(&project_save_state);

        state_system.build(cx);

        cx.add_theme(STYLE);

        VStack::new(cx, |cx|{
            // Top bar controls
            HStack::new(cx, |cx|{
                tempo_controls(cx);
                Element::new(cx).class("divider");
                transport_controls(cx);
            }).height(Pixels(70.0)).background_color(Color::rgb(63,57,59));

            // Tracks View
            tracks_view(cx);
        });


    })
    .on_idle(|cx|{
        cx.emit(AppEvent::Sync);
    })
    .run();
}