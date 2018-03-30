//
// COPYRIGHT NOTICE
//
// This code was adapted from alamminsalo/rust-metronome.
// https://github.com/alamminsalo/rust-metronome?files=1
//

use ears::{Sound, AudioController};
use errors::*;
use std::fmt;
use std::sync::mpsc::{channel, Sender, TryRecvError};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::{Instant, Duration};
use track::Player;

const ACCENT_PATH: &str = "./accent.wav";
const BEAT_PATH: &str = "./beat.wav";

pub struct Metronome {
    tempo: u16,
    signature: String,
    tx: Sender<bool>,
    _handle: JoinHandle<()>,
}

impl Metronome {
    pub fn new(tempo: u16, signature: &str) -> Result<Metronome> {
        // Parse time signature
        let sig_pair: Vec<&str> = signature.split_terminator('/').collect();
        let sig: (u8, u8) = (sig_pair[0].parse().unwrap(), sig_pair[1].parse().unwrap());

        // Spawn player thread
        let (tx, rx) = channel();
        let handle = spawn(move || {
            let mut snd_accent = Sound::new(&ACCENT_PATH).expect("Could not find metronome accent sound");
            let mut snd_beat = Sound::new(&BEAT_PATH).expect("Could not find metronome accent sound");

            loop {
                match rx.recv() {
                    Ok(b) if b == true => {
                        // Interval between beats
                        let interval = interval(tempo);

                        let mut beat: u8 = 0;
                        loop {
                            let t = Instant::now();

                            match rx.try_recv() {
                                Ok(_) | Err(TryRecvError::Disconnected) => break,
                                Err(TryRecvError::Empty) => ()
                            }

                            if beat == 0 {
                                snd_accent.play();
                            } else {
                                snd_beat.play();
                            }

                            // Update bar marker
                            beat += 1;
                            if beat >= sig.0 {
                                beat = 0;
                            }

                            // Sleep for remaining time in interval
                            sleep(interval - t.elapsed());
                        }
                    }
                    Err(_) => break,
                    _ => (),
                }
            }
        });

        Ok(Metronome {
            tempo: tempo,
            signature: signature.into(),
            tx: tx,
            _handle: handle
        })
    }
}

impl Player for Metronome {
    fn play(&self) -> Result<()> {
        Ok(self.tx.send(true)?)
    }

    fn pause(&self) -> Result<()> {
        self.stop()
    }

    fn stop(&self) -> Result<()> {
        Ok(self.tx.send(false)?)
    }
}

impl fmt::Display for Metronome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Metronome at {}bpm in {}", self.tempo, self.signature)
    }
}

fn interval(tempo: u16) -> Duration {
    let bignum: u64 = (60.0 / (tempo as f64) * 1_000_000_000.0) as u64;

    let seconds: u64 = bignum / 1_000_000_000;
    let nanoseconds: u32 = (bignum % 1_000_000_000) as u32;

    Duration::new(seconds, nanoseconds)
}