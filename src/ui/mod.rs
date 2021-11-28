use vizia::*;
use image::GenericImageView;

use crate::state::{AppEvent, ProjectSaveState, StateSystem};

mod tempo_controls;
use tempo_controls::tempo_controls;

mod transport_controls;
use transport_controls::transport_controls;

mod tracks_view;
use tracks_view::tracks_view;

mod track_controls;
pub use track_controls::*;

mod track;
pub use track::*;

mod loop_region;
pub use loop_region::*;

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

    .play_button {
        border-width: 0px;
        font-size: 24px;
        color: #f54e47;
    }
"#;


pub fn run() {

    let icon = image::open("./assets/branding/meadowlark-logo-32.png").unwrap();

    let window_description = WindowDescription::new()
        .with_title("Meadowlark")
        .with_inner_size(1280, 720)
        .with_icon(icon.to_bytes(), icon.width(), icon.height());

    let app = Application::new(window_description, |cx|{
        let project_save_state = Box::new(ProjectSaveState::test());
        let mut state_system = StateSystem::new();
        state_system.load_project(&project_save_state);

        state_system.build(cx);

        cx.add_theme(STYLE);

        VStack::new(cx, |cx|{
            // Top bar controls
            HStack::new(cx, |cx|{
                tempo_controls(cx).width(Pixels(300.0));
                Element::new(cx).class("divider");
                transport_controls(cx);
            }).height(Pixels(70.0)).background_color(Color::rgb(63,57,59)).bottom(Pixels(1.0));

            // Tracks View
            tracks_view(cx);
        }).background_color(Color::rgb(10, 10, 10));


    });

    let proxy = app.get_proxy();

    std::thread::spawn(move ||{
        loop {
            proxy.send_event(Event::new(AppEvent::Sync)).expect("Failed to send proxy event");
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    });

    // .on_idle(|cx|{
    //     cx.emit(AppEvent::Sync);
    // })
    app.run();
}