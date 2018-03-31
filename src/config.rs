#[derive(Deserialize)]
pub struct Config {
    pub tracks: Vec<ConfigTrack>,
    pub metronome: Option<ConfigMetronome>,
}

#[derive(Deserialize)]
pub struct ConfigTrack {
    #[serde(rename = "type")]
    pub track_type: TrackType,
    pub name: String,
    pub tempo: Option<u16>,
    pub signature: Option<String>,
    pub path: Option<String>,
    pub autostart: Option<bool>,
    pub delay: Option<u64>,
}

#[derive(Deserialize)]
pub struct ConfigMetronome {
    pub accent: String,
    pub beat: String,
}

#[derive(Deserialize)]
pub enum TrackType {
    #[serde(rename = "backing")]
    Backing,
    #[serde(rename = "metronome")]
    Metronome
}
