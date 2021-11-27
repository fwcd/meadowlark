use rusty_daw_core::{MusicalTime, SampleRate, Seconds};
use vizia::{Lens, Model};
use std::path::PathBuf;

use crate::backend::timeline::{AudioClipSaveState, LoopState};

/// This struct should contain all state that the UI will bind to. This should
/// mirror the `ProjectSaveState` plus whatever extra state is needed for the UI.
///
/// (Yes we are duplicating state from `ProjectSaveState`). This is for a couple
/// of reasons:
///
/// 1. This separates areas of concerns, so the UI can be developed independently
/// of the backend.
/// 2. Even if a project is not loaded in the backend, the UI should still show
/// something in its place like empty tracks and mixers.
/// 3. This makes it clearer what state the GUI cares about by consolidating all
/// state into the `ui_state` folder (as apposed to state being scattered around
/// the backend and various other 3rd party crates).
/// 4. This will make it easier to create "bindings/lenses" for data-driven UI
/// paridigms.
/// 5. This `UiState` struct is only exposed to the UI as an immutable reference
/// via the `StateSystem` struct. This ensures that any mutation of state *must*
/// go through the `StateSystem` struct which is responsible for keeping
/// everything in sync.
/// 6. Memory is cheap nowadays anyway, and it's not like we're cloning large
/// blocks of data like audio samples (the largest things we will clone will
/// mostly just be strings, piano roll clips, and automation tracks).
#[derive(Lens)]
pub struct UiState {
    pub backend_loaded: bool,

    pub timeline_transport: TimelineTransportUiState,
    pub tempo_map: TempoMapUiState,
    pub sample_rate: SampleRate,

    pub timeline_tracks: Vec<TimelineTrackUiState>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            backend_loaded: false,

            timeline_transport: TimelineTransportUiState::default(),
            tempo_map: TempoMapUiState { bpm: 110.0 },

            timeline_tracks: Vec::new(),
            sample_rate: SampleRate::new(Default::default()),
        }
    }
}

impl Model for UiState {
    
}

#[derive(Lens)]
pub struct TempoMapUiState {
    // TODO: This will need to change once we start to support automation of tempo.
    pub bpm: f64,
}

impl Model for TempoMapUiState {

}


#[derive(Lens)]
pub struct TimelineTransportUiState {
    pub is_playing: bool,
    /// The place where the playhead will seek to on project load/transport stop.
    pub seek_to: MusicalTime,
    pub loop_state: LoopUiState,
    pub playhead: MusicalTime,
}

impl Model for TimelineTransportUiState {

}

impl Default for TimelineTransportUiState {
    fn default() -> Self {
        Self {
            is_playing: false,
            seek_to: MusicalTime::new(0.into()),
            loop_state: LoopUiState::Inactive,
            playhead: MusicalTime::new(0.into()),
        }
    }
}

/// The status of looping on this transport.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopUiState {
    /// The transport is not currently looping.
    Inactive,
    /// The transport is currently looping.
    Active {
        /// The start of the loop (inclusive).
        loop_start: MusicalTime,
        /// The end of the loop (exclusive).
        loop_end: MusicalTime,
    },
}

impl From<LoopState> for LoopUiState {
    fn from(l: LoopState) -> Self {
        match l {
            LoopState::Inactive => LoopUiState::Inactive,
            LoopState::Active { loop_start, loop_end } => {
                LoopUiState::Active { loop_start, loop_end }
            }
        }
    }
}

#[derive(Lens)]
pub struct TimelineTrackUiState {
    /// The name displayed on this timeline track.
    pub name: String,

    /// The audio clips on this timeline track. These may not be
    /// in any particular order.
    pub audio_clips: Vec<AudioClipUiState>,
}

impl Model for TimelineTrackUiState {

}


#[derive(Lens)]
pub struct AudioClipUiState {
    /// The name displayed on the audio clip.
    pub name: String,

    /// The path to the audio file containing the PCM data.
    pub pcm_path: PathBuf,

    /// Where the clip starts on the timeline.
    pub timeline_start: MusicalTime,

    /// The duration of the clip on the timeline.
    pub duration: Seconds,

    /// The offset in the pcm resource where the "start" of the clip should start playing from.
    pub clip_start_offset: Seconds,

    /// The gain of the audio clip in decibels.
    pub clip_gain_db: f32,

    /// The fades on this audio clip.
    pub fades: AudioClipFadesUiState,
}

impl Model for AudioClipUiState {

}

impl From<&AudioClipSaveState> for AudioClipUiState {
    fn from(a: &AudioClipSaveState) -> Self {
        Self {
            name: a.name.clone(),
            pcm_path: a.pcm_path.clone(),
            timeline_start: a.timeline_start,
            duration: a.duration,
            clip_start_offset: a.clip_start_offset,
            clip_gain_db: a.clip_gain_db,
            fades: AudioClipFadesUiState {
                start_fade_duration: a.fades.start_fade_duration,
                end_fade_duration: a.fades.end_fade_duration,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Lens)]
pub struct AudioClipFadesUiState {
    pub start_fade_duration: Seconds,
    pub end_fade_duration: Seconds,
}

impl Model for AudioClipFadesUiState {
    
}
