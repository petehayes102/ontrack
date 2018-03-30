use ears::{AudioController, Music, State};
use errors::*;
use std::fmt;
use track::Player;

pub struct Backing {
    name: String,
    path: String,
    autostart: bool,
    music: Music,
}

impl Backing {
    pub fn new(name: &str, path: &str, autostart: bool) -> Result<Backing> {
        Ok(Backing {
            name: name.into(),
            path: path.into(),
            autostart: autostart,
            music: Music::new(&path)?,
        })
    }
}

impl Player for Backing {
    fn name(&self) -> &str {
        &self.name
    }

    fn play(&mut self) -> Result<()> {
        Ok(self.music.play())
    }

    fn pause(&mut self) -> Result<()> {
        Ok(self.music.pause())
    }

    fn stop(&mut self) -> Result<()> {
        Ok(self.music.stop())
    }

    fn get_state(&self) -> State {
        self.music.get_state()
    }

    fn autostart(&self) -> bool {
        self.autostart
    }
}

impl fmt::Display for Backing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Backing track - {}", self.path)
    }
}
