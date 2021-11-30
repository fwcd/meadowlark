use std::collections::VecDeque;

use cpal::Stream;
use rusty_daw_audio_graph::{NodeRef, PortType};
use rusty_daw_core::{MusicalTime, SampleRate};
use vizia::{Lens, Model};

use crate::backend::timeline::{LoopState, TimelineTrackHandle, TimelineTrackNode};
use crate::backend::{BackendCoreHandle, BackendCoreState, ResourceLoadError};

use super::ui_state::{LoopUiState, TimelineTrackUiState, UiState};
use super::ProjectSaveState;

pub struct Project {
    stream: Stream,
    save_state: ProjectSaveState,
    backend_core_handle: BackendCoreHandle,
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
        Self { project: None, ui_state: UiState::default(), undo_stack: VecDeque::new() }
    }

    pub fn get_ui_state(&self) -> &UiState {
        &self.ui_state
    }

    pub fn load_project(&mut self, project_state: &Box<ProjectSaveState>) {
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
            backend_core: project_state.backend_core.clone_with_sample_rate(sample_rate),
            timeline_tracks: Vec::with_capacity(project_state.timeline_tracks.len()),
        };

        let (mut backend_core_handle, rt_state) =
            BackendCoreHandle::from_state(sample_rate, &mut save_state.backend_core);

        let mut timeline_track_handles: Vec<(NodeRef, TimelineTrackHandle)> = Vec::new();
        let mut resource_load_errors: Vec<ResourceLoadError> = Vec::new();

        //This function is temporary. Eventually we should use rusty-daw-io instead.
        if let Ok(stream) = crate::backend::rt_thread::run_with_default_output(rt_state) {
            self.ui_state.tempo_map.bpm = project_state.backend_core.tempo_map.bpm();
            self.ui_state.sample_rate = sample_rate;
            self.ui_state.timeline_transport.seek_to =
                project_state.backend_core.timeline_transport.seek_to;
            self.ui_state.timeline_transport.loop_state =
                project_state.backend_core.timeline_transport.loop_state.into();

            // TODO: errors and reverting to previous working state
            backend_core_handle
                .modify_graph(|mut graph, resource_cache| {
                    let root_node_ref = graph.root_node();

                    for timeline_track_state in project_state.timeline_tracks.iter() {
                        // --- Load timeline track in backend ----------------------

                        save_state.timeline_tracks.push(timeline_track_state.clone());

                        let (timeline_track_node, timeline_track_handle, mut res) =
                            TimelineTrackNode::new(
                                timeline_track_state,
                                resource_cache,
                                &project_state.backend_core.tempo_map,
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
                            name: timeline_track_state.name.clone(),
                            height: 150.0,
                            audio_clips: timeline_track_state
                                .audio_clips
                                .iter()
                                .map(|s| s.into())
                                .collect(),
                        });
                    }
                })
                .unwrap();

            self.project =
                Some(Project { stream, save_state, backend_core_handle, timeline_track_handles });

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
            project.backend_core_handle.set_bpm(bpm, &mut project.save_state.backend_core);
        }
    }

    // FIX ME
    pub fn set_loop_end(&mut self, new_loop_end: MusicalTime) {
        let loop_start = match &mut self.ui_state.timeline_transport.loop_state {
            LoopUiState::Active { loop_start, loop_end } => {
                *loop_end = new_loop_end;
                *loop_start
            }

            LoopUiState::Inactive => {
                self.ui_state.timeline_transport.loop_state =
                    LoopUiState::Active { loop_start: new_loop_end, loop_end: new_loop_end };

                new_loop_end
            }
        };

        if let Some(project) = &mut self.project {
            let (transport, _) = project
                .backend_core_handle
                .timeline_transport_mut(&mut project.save_state.backend_core);
            if let Err(_) = transport.set_loop_state(
                LoopState::Active { loop_start, loop_end: new_loop_end },
                &mut project.save_state.backend_core.timeline_transport,
            ) {
                // TODO: Handle this.
            }
        }
    }

    // FIX ME
    pub fn set_loop_start(&mut self, new_loop_start: MusicalTime) {
        let loop_end = match &mut self.ui_state.timeline_transport.loop_state {
            LoopUiState::Active { loop_start, loop_end } => {
                *loop_start = new_loop_start;
                *loop_end
            }

            LoopUiState::Inactive => {
                self.ui_state.timeline_transport.loop_state =
                    LoopUiState::Active { loop_start: new_loop_start, loop_end: new_loop_start };

                new_loop_start
            }
        };

        if let Some(project) = &mut self.project {
            let (transport, _) = project
                .backend_core_handle
                .timeline_transport_mut(&mut project.save_state.backend_core);
            if let Err(_) = transport.set_loop_state(
                LoopState::Active { loop_start: new_loop_start, loop_end },
                &mut project.save_state.backend_core.timeline_transport,
            ) {
                // TODO: Handle this.
            }
        }
    }

    // Set the start position of a clip in musical time
    pub fn set_clip_start(&mut self, track_id: usize, clip_id: usize, timeline_start: MusicalTime) {
        if let Some(track_state) = self.ui_state.timeline_tracks.get_mut(track_id) {
            if let Some(clip_state) = track_state.audio_clips.get_mut(clip_id) {
                clip_state.timeline_start = timeline_start;
            }
        }

        if let Some(project) = &mut self.project {
            if let Some((_, track)) = project.timeline_track_handles.get_mut(track_id) {
                let (tempo_map, timeline_tracks_state) = project.save_state.timeline_tracks_mut();
                if let Some((clip_handle, clip_state)) =
                    track.audio_clip_mut(clip_id, timeline_tracks_state.get_mut(track_id).unwrap())
                {
                    clip_handle.set_timeline_start(timeline_start, tempo_map, clip_state);
                }
            }
        }
    }

    pub fn timeline_transport_play(&mut self) {
        if let Some(project) = &mut self.project {
            if !self.ui_state.timeline_transport.is_playing {
                self.ui_state.timeline_transport.is_playing = true;

                let (transport, _) = project
                    .backend_core_handle
                    .timeline_transport_mut(&mut project.save_state.backend_core);
                transport.set_playing(true);
            }
        }
    }

    pub fn timeline_transport_pause(&mut self) {
        if let Some(project) = &mut self.project {
            if self.ui_state.timeline_transport.is_playing {
                self.ui_state.timeline_transport.is_playing = false;

                let (transport, _) = project
                    .backend_core_handle
                    .timeline_transport_mut(&mut project.save_state.backend_core);
                transport.set_playing(false);
            }
        }
    }

    /// Switch the timeline transport state between playing and paused
    pub fn timeline_transport_play_pause(&mut self) {
        if let Some(project) = &mut self.project {
            if self.ui_state.timeline_transport.is_playing {
                self.ui_state.timeline_transport.is_playing = false;

                let (transport, _) = project
                    .backend_core_handle
                    .timeline_transport_mut(&mut project.save_state.backend_core);
                transport.set_playing(false);
            } else {
                self.ui_state.timeline_transport.is_playing = true;
                let (transport, _) = project
                    .backend_core_handle
                    .timeline_transport_mut(&mut project.save_state.backend_core);
                transport.set_playing(true);
            }
        }
    }

    pub fn timeline_transport_stop(&mut self) {
        self.ui_state.timeline_transport.is_playing = false;
        self.ui_state.timeline_transport.seek_to = 0.0.into();

        if let Some(project) = &mut self.project {
            let (transport, transport_state) = project
                .backend_core_handle
                .timeline_transport_mut(&mut project.save_state.backend_core);
            transport.set_playing(false);
            transport.seek_to(0.0.into(), transport_state);
        }
    }

    pub fn sync_playhead(&mut self) {
        if let Some(project) = &mut self.project {
            let (transport, _) = project
                .backend_core_handle
                .timeline_transport_mut(&mut project.save_state.backend_core);
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
    PlayPause,
    Stop,

    // Loop Controls
    SetLoopStart(MusicalTime),
    SetLoopEnd(MusicalTime),

    // Track Controls
    SetTrackHeight(usize, f32),

    // Clip Controls
    // TODO - create types for track id and clip id
    SetClipStart(usize, usize, MusicalTime),
}

impl Model for StateSystem {
    fn event(&mut self, cx: &mut vizia::Context, event: &mut vizia::Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::Sync => {
                    self.sync_playhead();
                }

                // TEMPO
                AppEvent::SetBpm(bpm) => {
                    self.set_bpm(*bpm);
                }

                // TRANSPORT
                AppEvent::Play => {
                    self.timeline_transport_play();
                    self.sync_playhead();
                }

                AppEvent::Pause => {
                    self.timeline_transport_pause();
                    self.sync_playhead();
                }

                AppEvent::PlayPause => {
                    self.timeline_transport_play_pause();
                    self.sync_playhead();
                }

                AppEvent::Stop => {
                    self.timeline_transport_stop();
                    self.sync_playhead();
                }

                // LOOP
                AppEvent::SetLoopStart(loop_start) => {
                    self.set_loop_start(*loop_start);
                }

                AppEvent::SetLoopEnd(loop_end) => {
                    self.set_loop_end(*loop_end);
                }

                // TRACK
                AppEvent::SetTrackHeight(track_id, track_height) => {
                    if let Some(track_state) = self.ui_state.timeline_tracks.get_mut(*track_id) {
                        track_state.height = *track_height;
                    }
                }

                // CLIP
                AppEvent::SetClipStart(track_id, clip_id, timeline_start) => {
                    let timeline_start = MusicalTime::new(timeline_start.0.max(0.0));
                    self.set_clip_start(*track_id, *clip_id, timeline_start);
                }
            }
        }
    }
}
