use serde::{Deserialize, Serialize};

use crate::player::track::Track;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistoryUpdateEvent {
    pub has_history: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrackChangedEvent {
    pub track: Option<Track>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QueueChangedEvent {
    pub queue: Vec<Track>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEvent {
    pub position: f64,
    pub frames_played: u64,
    pub spectrum_data: Vec<f32>,
}
