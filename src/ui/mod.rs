use crate::state::{
    event::{ProjectEvent, StateSystemEvent},
    BoundGuiState, ProjectSaveState, StateSystem,
};

pub mod views;
use views::*;

use vizia::*;

const THEME: &str = include_str!("theme.css");

pub struct App {

}

impl App {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx).background_color(Color::rgb(10,10,10))
    }
}

impl View for App {
    fn body(&mut self, cx: &mut Context) {
        Header::new(cx);

        //app.set_background_color(state, Color::rgb(10, 10, 10))
    }
}

pub fn run() {
    let project_save_state = Box::new(ProjectSaveState::test());

    let window_description = WindowDescription::new().with_title("Meadowlark");
    let app = Application::new(move |cx| {

        cx.add_theme(THEME);

        BoundGuiState::new().build(cx);

        cx.emit_trace(StateSystemEvent::Project(ProjectEvent::LoadProject(project_save_state.clone())));

        App::new(cx);

    });

    app.run();
}
