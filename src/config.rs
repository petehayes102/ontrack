#[derive(Deserialize)]
pub struct Config {
    pub tracks: Vec<ConfigTrack>
}

#[derive(Deserialize)]
pub struct ConfigTrack {
    #[serde(rename = "type")]
    pub track_type: TrackType,
    pub name: String,
    pub tempo: Option<u16>,
    pub signature: Option<String>,
}

#[derive(Deserialize)]
pub enum TrackType {
    #[serde(rename = "backing")]
    Backing,
    #[serde(rename = "metronome")]
    Metronome
}
