use backing::Backing;
use config::{Config, TrackType};
use ears::State;
use errors::*;
use metronome::Metronome;
use std::fmt;
use std::io::{Stdout, Write, stdout};
use termion;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Tracks {
    tracks: Vec<Track>,
    index: usize,
    stdout: RawTerminal<Stdout>,
}

pub enum Track {
    Backing(Backing),
    Metronome(Metronome),
}

pub trait Player {
    fn name(&self) -> &str;
    fn play(&mut self) -> Result<()>;
    fn pause(&mut self) -> Result<()>;
    fn stop(&mut self) -> Result<()>;
    fn get_state(&self) -> State;
    fn autostart(&self) -> bool;
}

impl Tracks {
    pub fn from_config(config: Config) -> Result<Tracks> {
        let mut tracks = Vec::new();

        for track in config.tracks {
            let t = match track.track_type {
                TrackType::Backing => {
                    let backing = Backing::new(&track.name, &track.path.expect("Track path missing"), track.autostart.unwrap_or(false))?;
                    Track::Backing(backing)
                },
                TrackType::Metronome => {
                    let tempo = track.tempo.expect("Tempo missing");
                    let signature = track.signature.expect("Time signature missing");
                    let met = config.metronome.as_ref().expect("Metronome config missing");
                    let metronome = Metronome::new(&track.name, tempo, &signature, track.autostart.unwrap_or(false), &met.accent, &met.beat)?;
                    Track::Metronome(metronome)
                }
            };
            tracks.push(t);
        }

        Ok(Tracks {
            tracks: tracks,
            index: 0,
            stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn next(&mut self) -> Result<()> {
        if self.index < self.tracks.len()-1 {
            self.stop()?;
            self.index += 1;
            self.announce_track();
        }

        Ok(())
    }

    pub fn previous(&mut self) -> Result<()> {
        if self.index > 0 {
            self.stop()?;
            self.index -= 1;
            self.announce_track();
        }

        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        let r = self.active_track().play();
        self.announce_track();
        r
    }

    pub fn pause(&mut self) -> Result<()> {
        let r = self.active_track().pause();
        self.announce_track();
        r
    }

    pub fn stop(&mut self) -> Result<()> {
        let r = self.active_track().stop();
        self.announce_track();
        r
    }

    pub fn play_pause(&mut self) -> Result<()> {
        match self.active_track().get_state() {
            State::Initial => self.play(),
            State::Playing => self.pause(),
            State::Paused => self.play(),
            State::Stopped => self.play(),
        }
    }

    fn active_track(&mut self) -> &mut Track {
        self.tracks.get_mut(self.index)
            .expect("index is outside array bounds!")
    }

    pub fn announce_track(&mut self) {
        let (name, track, state) = {
            let active = self.active_track();
            (
                active.name().to_owned(),
                format!("{}", active),
                match active.get_state() {
                    State::Initial => "Stopped",
                    State::Playing => "Playing",
                    State::Paused => "Paused",
                    State::Stopped => "Stopped",
                }
            )
        };

        write!(self.stdout, "{}{}{}Track {} ({}){}: {} [{}{}{}]{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::color::Fg(termion::color::Green),
            self.index + 1,
            name,
            termion::color::Fg(termion::color::Reset),
            track,
            termion::color::Fg(termion::color::Red),
            state,
            termion::color::Fg(termion::color::Reset),
            termion::cursor::Hide).unwrap();

        self.stdout.flush().unwrap();
    }
}

impl Drop for Tracks {
    fn drop(&mut self) {
        write!(self.stdout, "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            termion::cursor::Show).unwrap();
    }
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

    fn get_state(&self) -> State {
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
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Track::Backing(ref b) => write!(f, "{}", b),
            Track::Metronome(ref m) => write!(f, "{}", m),
        }
    }
}
