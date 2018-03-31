use backing::Backing;
use ears::State;
use errors::*;
use metronome::Metronome;
use std::fmt;

pub enum Track {
    Backing(Backing),
    Metronome(Metronome),
}

pub trait Player {
    fn name(&self) -> &str;
    fn play(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn get_state(&self) -> Result<State>;
    fn autostart(&self) -> bool;
    fn set_autostart(&mut self, bool);
}

impl Player for Track {
    fn name(&self) -> &str {
        match *self {
            Track::Backing(ref b) => b.name(),
            Track::Metronome(ref m) => m.name(),
        }
    }
    fn play(&mut self) -> Result<()> {
        match *self {
            Track::Backing(ref mut b) => b.play(),
            Track::Metronome(ref mut m) => m.play(),
        }
    }

    fn pause(&mut self) -> Result<()> {
        match *self {
            Track::Backing(ref mut b) => b.pause(),
            Track::Metronome(ref mut m) => m.pause(),
        }
    }

    fn stop(&mut self) -> Result<()> {
        match *self {
            Track::Backing(ref mut b) => b.stop(),
            Track::Metronome(ref mut m) => m.stop(),
        }
    }

    fn get_state(&self) -> Result<State> {
        match *self {
            Track::Backing(ref b) => b.get_state(),
            Track::Metronome(ref m) => m.get_state(),
        }
    }

    fn autostart(&self) -> bool {
        match *self {
            Track::Backing(ref b) => b.autostart(),
            Track::Metronome(ref m) => m.autostart(),
        }
    }

    fn set_autostart(&mut self, autostart: bool) {
        match *self {
            Track::Backing(ref mut b) => b.set_autostart(autostart),
            Track::Metronome(ref mut m) => m.set_autostart(autostart),
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Track::Backing(ref b) => write!(f, "{}", b),
            Track::Metronome(ref m) => write!(f, "{}", m),
        }
    }
}
