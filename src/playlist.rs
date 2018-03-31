use backing::Backing;
use config::{Config, TrackType};
use ears::State;
use errors::*;
use metronome::Metronome;
use std::io::{Stdout, Write, stdout};
use std::sync::mpsc::{channel, Sender};
use std::thread::{JoinHandle, spawn};
use termion;
use termion::raw::{IntoRawMode, RawTerminal};
use track::{Player, Track};

#[derive(Clone)]
pub struct Playlist {
    tx: Sender<u8>,
}

struct Internal {
    tracks: Vec<Track>,
    index: usize,
    stdout: RawTerminal<Stdout>,
}

impl Playlist {
    pub fn from_config(config: Config) -> Result<(Playlist, JoinHandle<()>)> {
        let (tx, rx) = channel();
        let playlist = Playlist {
            tx: tx,
        };

        let mut tracks = Vec::new();

        for track in config.tracks {
            let t = match track.track_type {
                TrackType::Backing => {
                    let mut backing = Backing::new(&track.name, &track.path.expect("Track path missing"), playlist.clone())?;
                    backing.set_autostart(track.autostart.unwrap_or(false));
                    Track::Backing(backing)
                },
                TrackType::Metronome => {
                    let tempo = track.tempo.expect("Tempo missing");
                    let signature = track.signature.expect("Time signature missing");
                    let met = config.metronome.as_ref().expect("Metronome config missing");
                    let mut metronome = Metronome::new(&track.name, tempo, &signature, &met.accent, &met.beat, playlist.clone())?;
                    metronome.set_autostart(track.autostart.unwrap_or(false));
                    Track::Metronome(metronome)
                }
            };
            tracks.push(t);
        }

        let stdout = stdout().into_raw_mode()?;

        let handle = spawn(move || {
            let mut internal = Internal {
                tracks: tracks,
                index: 0,
                stdout: stdout,
            };

            // Announce the first track, just to be nice!
            internal.announce_track();

            loop {
                match rx.recv() {
                    Ok(cmd) => {
                        match cmd {
                            0 => internal.stop(),
                            1 => internal.play_pause(),
                            2 => internal.previous(),
                            3 => internal.next(),
                            4 => internal.finished(),
                            _ => unreachable!(),
                        }
                    },
                    Err(_) => break,
                }
            }
        });

        Ok((playlist, handle))
    }
}

impl Playlist {
    pub fn next(&self) -> Result<()> {
        Ok(self.tx.send(3)?)
    }

    pub fn previous(&self) -> Result<()> {
        Ok(self.tx.send(2)?)
    }

    pub fn stop(&self) -> Result<()> {
        Ok(self.tx.send(0)?)
    }

    pub fn play_pause(&self) -> Result<()> {
        Ok(self.tx.send(1)?)
    }

    pub fn finished(&self) -> Result<()> {
        Ok(self.tx.send(4)?)
    }
}

impl Internal {
    fn next(&mut self) {
        if self.index < self.tracks.len()-1 {
            self.stop();
            self.index += 1;
            self.announce_track();
        }
    }

    fn previous(&mut self) {
        if self.index > 0 {
            self.stop();
            self.index -= 1;
            self.announce_track();
        }
    }

    fn play(&mut self) {
        self.active_track().play().unwrap();
        self.announce_track();
    }

    fn pause(&mut self) {
        self.active_track().pause().unwrap();
        self.announce_track();
    }

    fn stop(&mut self) {
        self.active_track().stop().unwrap();
        self.announce_track();
    }

    fn play_pause(&mut self) {
        match self.active_track().get_state().unwrap() {
            State::Initial => self.play(),
            State::Playing => self.pause(),
            State::Paused => self.play(),
            State::Stopped => self.play(),
        }
    }

    // When the current track finishes, autoplay the next track if necessary
    fn finished(&mut self) {
        let mut autostart = false;
        if let Some(t) = self.tracks.get(self.index + 1) {
            autostart = t.autostart();
        }

        if autostart {
            self.next();
            self.play();
        }
    }

    fn announce_track(&mut self) {
        let (name, track, state) = {
            let active = self.active_track();
            (
                active.name().to_owned(),
                format!("{}", active),
                match active.get_state().unwrap() {
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

    fn active_track(&mut self) -> &mut Track {
        self.tracks.get_mut(self.index)
            .expect("index is outside array bounds!")
    }
}
