use vizia::{Entity, Event, Lens, Model, State, Context};

use super::{ProjectSaveState, StateSystem, project_save_state};

#[derive(Lens)]
pub struct BoundGuiState {
    #[lens(ignore)]
    pub state_system: Option<StateSystem>,

    pub save_state: ProjectSaveState,

    pub backend_loaded: bool,
    pub is_playing: bool,
    pub bpm: f64,
}

impl BoundGuiState {
    pub fn new() -> Self {
        Self {
            state_system: Some(StateSystem::new()),
            save_state: ProjectSaveState::new_empty(),
            backend_loaded: false,
            is_playing: false,
            bpm: 110.0,
        }
    }
}

impl Model for BoundGuiState {
    fn event(&mut self, cx: &mut Context, event: &mut Event) -> bool {
        if let Some(state_system_event) = event.message.downcast() {
            // This is to get around the borrow checker.
            let mut state_system = self.state_system.take().unwrap();

            let ret = state_system.on_event(self, cx, state_system_event);

            self.state_system = Some(state_system);

            ret
        } else {
            false
        }
    }
}
