// use backing::Backing;
use config::{Config, TrackType};
use errors::*;
use metronome::Metronome;
use std::fmt;
use std::io::{Stdout, Write, stdout};
use termion;
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Tracks {
    tracks: Vec<Track>,
    index: usize,
    state: TrackState,
    stdout: RawTerminal<Stdout>,
}

pub enum Track {
    // Backing(Backing),
    Backing,
    Metronome(Metronome),
}

pub enum TrackState {
    Play,
    Pause,
    Stop,
}

pub trait Player {
    fn play(&self) -> Result<()>;
    fn pause(&self) -> Result<()>;
    fn stop(&self) -> Result<()>;
}

impl Tracks {
    pub fn from_config(config: Config) -> Result<Tracks> {
        let mut tracks = Vec::new();

        for track in config.tracks {
            let t = match track.track_type {
                TrackType::Backing => unimplemented!(),
                TrackType::Metronome => {
                    let tempo = track.tempo.expect("Tempo missing");
                    let signature = track.signature.expect("Time signature missing");
                    let metronome = Metronome::new(tempo, &signature)?;
                    Track::Metronome(metronome)
                }
            };
            tracks.push(t);
        }

        Ok(Tracks {
            tracks: tracks,
            index: 0,
            state: TrackState::Stop,
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
        self.state = TrackState::Play;
        self.announce_track();
        self.active_track().play()
    }

    pub fn pause(&mut self) -> Result<()> {
        self.state = TrackState::Pause;
        self.announce_track();
        self.active_track().pause()
    }

    fn stop(&mut self) -> Result<()> {
        self.state = TrackState::Stop;
        self.announce_track();
        self.active_track().stop()
    }

    pub fn play_pause(&mut self) -> Result<()> {
        match self.state {
            TrackState::Play => self.pause(),
            TrackState::Pause => self.play(),
            TrackState::Stop => self.play(),
        }
    }

    fn active_track(&self) -> &Track {
        self.tracks.get(self.index)
            .expect("index is outside array bounds!")
    }

    pub fn announce_track(&mut self) {
        let active = format!("{}", self.active_track());

        write!(self.stdout, "{}{}Track {}: {} [{}]{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            self.index + 1,
            active,
            self.state,
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
    fn play(&self) -> Result<()> {
        match *self {
            Track::Backing => unimplemented!(),
            Track::Metronome(ref m) => m.play(),
        }
    }

    fn pause(&self) -> Result<()> {
        match *self {
            Track::Backing => unimplemented!(),
            Track::Metronome(ref m) => m.pause(),
        }
    }

    fn stop(&self) -> Result<()> {
        match *self {
            Track::Backing => unimplemented!(),
            Track::Metronome(ref m) => m.stop(),
        }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Track::Backing => write!(f, "Backing"),
            Track::Metronome(ref m) => write!(f, "{}", m),
        }
    }
}

impl fmt::Display for TrackState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TrackState::Play => write!(f, "Playing"),
            TrackState::Pause => write!(f, "Paused"),
            TrackState::Stop => write!(f, "Stopped"),
        }
    }
}
