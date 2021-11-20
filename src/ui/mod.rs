use eframe::{egui, epi};

use crate::state::{ProjectSaveState, StateSystem};

/*
impl Widget for App {
    type Ret = Entity;
    type Data = ();
    fn on_build(&mut self, state: &mut State, app: Entity) -> Self::Ret {
        Header::default().build(state, app, |builder| builder);

        app.set_background_color(state, Color::rgb(10, 10, 10))
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {}
}
*/

/*
pub fn run() {
    let project_save_state = Box::new(ProjectSaveState::test());

    /*
    let window_description = WindowDescription::new().with_title("Meadowlark");
    let app = Application::new(window_description, |state, window| {
        //state.add_theme(DEFAULT_THEME);
        state.add_theme(THEME);

        //let text_to_speech = TextToSpeach::new().build(state, window, |builder| builder);

        let bound_gui_state = BoundGuiState::new().build(state, window);

        let app = App::new().build(state, bound_gui_state, |builder| builder);

        bound_gui_state
            .emit(state, StateSystemEvent::Project(ProjectEvent::LoadProject(project_save_state)));
    });

    app.run();
    */
}
*/

pub fn run() {
    let project_save_state = Box::new(ProjectSaveState::test());

    let meadowlark_app = MeadowlarkApp::new(project_save_state);

    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(meadowlark_app), options);
}

struct MeadowlarkApp {
    state_system: StateSystem,
}

impl MeadowlarkApp {
    fn new(project_save_state: Box<ProjectSaveState>) -> Self {
        let mut state_system = StateSystem::new();
        state_system.load_project(&project_save_state);
        Self { state_system }
    }
}

impl epi::App for MeadowlarkApp {
    fn name(&self) -> &str {
        "Meadowlark (egui prototype)"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").resizable(false).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("BPM");
                let mut bpm_value = self.state_system.ui_state().tempo_map.bpm;
                if ui
                    .add(egui::DragValue::new(&mut bpm_value).speed(1.0).fixed_decimals(1))
                    .changed()
                {
                    self.state_system.set_bpm(bpm_value);
                }

                ui.separator();

                if self.state_system.ui_state().timeline_transport.is_playing {
                    if ui.button("Pause").clicked() {
                        self.state_system.timeline_transport_pause();
                    }
                } else {
                    if ui.button("Play").clicked() {
                        self.state_system.timeline_transport_play();
                    }
                }

                if ui.button("Stop").clicked() {
                    self.state_system.timeline_transport_stop();
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
