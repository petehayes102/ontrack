use ears::{AudioController, Music, State};
use errors::*;
use playlist::Playlist;
use std::fmt;
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;
use track::Player;

pub struct Backing {
    name: String,
    path: String,
    autostart: bool,
    tx: Sender<u8>,
    rx: Receiver<State>,
    _handle: JoinHandle<()>,
}

impl Backing {
    pub fn new(name: &str, path: &str, delay: u64, playlist: Playlist) -> Result<Backing> {
        let mut music = Music::new(&path)?;
        let (tx, rx) = channel();
        let (state_tx, state_rx) = channel();

        let handle = spawn(move || {
            let mut started = false;
            loop {
                match rx.recv_timeout(Duration::from_millis(200)) {
                    Ok(cmd) => match cmd {
                        0 => {
                            started = false;
                            music.stop()
                        },
                        1 => {
                            sleep(Duration::from_secs(delay));
                            started = true;
                            music.play();
                        },
                        2 => {
                            started = false;
                            music.pause();
                        },
                        3 => state_tx.send(music.get_state()).unwrap(),
                        _ => unreachable!(),
                    },
                    Err(RecvTimeoutError::Timeout) if started && !music.is_playing() => {
                        started = false;
                        playlist.finished().unwrap();
                    },
                    Err(RecvTimeoutError::Disconnected) => break,
                    _ => (),
                }
            }
        });

        Ok(Backing {
            name: name.into(),
            path: path.into(),
            autostart: false,
            tx: tx,
            rx: state_rx,
            _handle: handle,
        })
    }
}

impl Player for Backing {
    fn name(&self) -> &str {
        &self.name
    }

    fn play(&mut self) -> Result<()> {
        Ok(self.tx.send(1)?)
    }

    fn pause(&mut self) -> Result<()> {
        Ok(self.tx.send(2)?)
    }

    fn stop(&mut self) -> Result<()> {
        Ok(self.tx.send(0)?)
    }

    fn get_state(&self) -> Result<State> {
        self.tx.send(3)?;
        Ok(self.rx.recv()?)
    }

    fn autostart(&self) -> bool {
        self.autostart
    }

    fn set_autostart(&mut self, autostart: bool) {
        self.autostart = autostart;
    }
}

impl fmt::Display for Backing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Backing track - {}", self.path)
    }
}
