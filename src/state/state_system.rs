use std::collections::VecDeque;

use cpal::Stream;
use rusty_daw_audio_graph::{NodeRef, PortType};
use rusty_daw_core::SampleRate;
use vizia::{Lens, Model};

use crate::backend::timeline::{TimelineTrackHandle, TimelineTrackNode};
use crate::backend::{BackendHandle, ResourceLoadError};

use super::ui_state::{TimelineTrackUiState, UiState};
use super::ProjectSaveState;

pub struct Project {
    stream: Stream,
    save_state: ProjectSaveState,
    backend_handle: BackendHandle,
    timeline_track_handles: Vec<(NodeRef, TimelineTrackHandle)>,
}

/// This struct is responsible for managing and mutating state of the entire application.
/// All mutation of any backend, UI, or save state must happen through this struct.
/// It is the responsibility of this struct to make sure all 3 of these states are synced
/// properly.
#[derive(Lens)]
pub struct StateSystem {
    #[lens(ignore)]
    project: Option<Project>,
    ui_state: UiState,
    undo_stack: VecDeque<AppEvent>,
}

impl StateSystem {
    pub fn new() -> Self {
        Self { 
            project: None,
            ui_state: UiState::default(),
            undo_stack: VecDeque::new(),
        }
    }

    pub fn get_ui_state(&self) -> &UiState {
        &self.ui_state
    }

    pub fn load_project(&mut self, project_save_state: &Box<ProjectSaveState>) {
        // This will drop and automatically close any already active project.
        self.project = None;

        // Reset the UI state:
        self.ui_state.backend_loaded = false;
        self.ui_state.timeline_transport.is_playing = false;
        self.ui_state.timeline_tracks.clear();

        // This function is temporary. Eventually we should use rusty-daw-io instead.
        let sample_rate =
            crate::backend::hardware_io::default_sample_rate().unwrap_or(SampleRate::default());

        let mut save_state = ProjectSaveState {
            backend: project_save_state.backend.clone_with_sample_rate(sample_rate),
            timeline_tracks: Vec::with_capacity(project_save_state.timeline_tracks.len()),
        };

        let (mut backend_handle, rt_state) =
            BackendHandle::from_save_state(sample_rate, &mut save_state.backend);

        let mut timeline_track_handles: Vec<(NodeRef, TimelineTrackHandle)> = Vec::new();
        let mut resource_load_errors: Vec<ResourceLoadError> = Vec::new();

        //This function is temporary. Eventually we should use rusty-daw-io instead.
        if let Ok(stream) = crate::backend::rt_thread::run_with_default_output(rt_state) {
            self.ui_state.tempo_map.bpm = project_save_state.backend.tempo_map.bpm();
            self.ui_state.sample_rate = sample_rate;
            self.ui_state.timeline_transport.seek_to =
                project_save_state.backend.timeline_transport.seek_to;
            self.ui_state.timeline_transport.loop_state =
                project_save_state.backend.timeline_transport.loop_state.into();

            // TODO: errors and reverting to previous working state
            backend_handle
                .modify_graph(|mut graph, resource_cache| {
                    let root_node_ref = graph.root_node();

                    for timeline_track_save_state in project_save_state.timeline_tracks.iter() {
                        // --- Load timeline track in backend ----------------------

                        save_state.timeline_tracks.push(timeline_track_save_state.clone());

                        let (timeline_track_node, timeline_track_handle, mut res) =
                            TimelineTrackNode::new(
                                timeline_track_save_state,
                                resource_cache,
                                &project_save_state.backend.tempo_map,
                                sample_rate,
                                graph.coll_handle(),
                            );

                        // Append any errors that happened while loading resources.
                        resource_load_errors.append(&mut res);

                        // Add the track node to the graph.
                        let timeline_track_node_ref =
                            graph.add_new_node(Box::new(timeline_track_node));

                        // Keep a reference and a handle to the track node.
                        timeline_track_handles
                            .push((timeline_track_node_ref, timeline_track_handle));

                        // Connect the track node to the root node.
                        //
                        // TODO: Handle errors.
                        graph
                            .connect_ports(
                                PortType::StereoAudio,
                                timeline_track_node_ref,
                                0,
                                root_node_ref,
                                0,
                            )
                            .unwrap();

                        // --- Load timeline track in UI ---------------------------

                        self.ui_state.timeline_tracks.push(TimelineTrackUiState {
                            name: timeline_track_save_state.name.clone(),
                            audio_clips: timeline_track_save_state
                                .audio_clips
                                .iter()
                                .map(|s| s.into())
                                .collect(),
                        });
                    }
                })
                .unwrap();

            self.project =
                Some(Project { stream, save_state, backend_handle, timeline_track_handles });

            self.ui_state.backend_loaded = true;
        } else {
            // TODO: Better errors
            log::error!("Failed to start audio stream");
            // TODO: Remove this panic
            panic!("Failed to start audio stream");
        }
    }

    pub fn set_bpm(&mut self, bpm: f64) {
        let bpm = if bpm <= 0.0 { 0.1 } else { bpm.clamp(0.0, 100_000.0) };
        self.ui_state.tempo_map.bpm = bpm;

        if let Some(project) = &mut self.project {
            project.backend_handle.set_bpm(bpm, &mut project.save_state.backend);
        }
    }

    pub fn timeline_transport_play(&mut self) {
        if let Some(project) = &mut self.project {
            if !self.ui_state.timeline_transport.is_playing {
                self.ui_state.timeline_transport.is_playing = true;

                let (transport, _) =
                    project.backend_handle.timeline_transport_mut(&mut project.save_state.backend);
                transport.set_playing(true);
            }
        }
    }

    pub fn timeline_transport_pause(&mut self) {
        if let Some(project) = &mut self.project {
            if self.ui_state.timeline_transport.is_playing {
                self.ui_state.timeline_transport.is_playing = false;

                let (transport, _) =
                    project.backend_handle.timeline_transport_mut(&mut project.save_state.backend);
                transport.set_playing(false);
            }
        }
    }

    pub fn timeline_transport_stop(&mut self) {
        self.ui_state.timeline_transport.is_playing = false;
        self.ui_state.timeline_transport.seek_to = 0.0.into();

        if let Some(project) = &mut self.project {
            let (transport, save_state) =
                project.backend_handle.timeline_transport_mut(&mut project.save_state.backend);
            transport.set_playing(false);
            transport.seek_to(0.0.into(), save_state);
        }
    }

    pub fn sync_playhead(&mut self) {
        if let Some(project) = &mut self.project {
            let (transport, save_state) =
                project.backend_handle.timeline_transport_mut(&mut project.save_state.backend);
            self.ui_state.timeline_transport.playhead = transport.get_playhead_position();
        }
    }
}


#[derive(Debug)]
pub enum AppEvent {

    // Force a sync
    Sync,

    // Tempo Controls
    SetBpm(f64),

    // Transport Controls
    Play,
    Pause,
    Stop,
}


impl Model for StateSystem {
    fn event(&mut self, cx: &mut vizia::Context, event: &mut vizia::Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {

                AppEvent::Sync => {
                    self.sync_playhead();
                }

                AppEvent::SetBpm(bpm) => {
                    println!("Set bpm: {}", bpm);
                    self.set_bpm(*bpm);
                }

                AppEvent::Play => {
                    println!("Play");
                    self.timeline_transport_play();
                    
                }

                AppEvent::Pause => {
                    println!("Pause");
                    self.timeline_transport_pause();
                    self.sync_playhead();
                }

                AppEvent::Stop => {
                    println!("Stop");
                    self.timeline_transport_stop();
                }
            }
        }
    }
}