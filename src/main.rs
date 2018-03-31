extern crate ears;
#[macro_use] extern crate error_chain;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate termion;
extern crate toml;

mod backing;
mod config;
mod errors;
mod metronome;
mod playlist;
mod track;

use config::Config;
use errors::*;
use std::env::args;
use std::fs::File;
use std::io::{Read, stdin};
use std::process::exit;
use termion::event::Key;
use termion::input::TermRead;
use playlist::Playlist;

quick_main!(|| -> Result<()> {
    let mut args = args();
    let path = match args.nth(1) {
        Some(p) => p,
        None => {
            println!("Usage: ontrack <playlist_path>");
            exit(1);
        },
    };

    // Load config
    let mut toml = String::new();
    let mut fh = File::open(&path)?;
    fh.read_to_string(&mut toml)?;
    let config: Config = toml::from_str(&toml)?;

    // Load Playlist
    let (playlist, _handle) = Playlist::from_config(config)?;

    let stdin = stdin();
    for c in stdin.keys() {
        match c {
            Ok(Key::Left) => playlist.previous()?,
            Ok(Key::Right) => playlist.next()?,
            Ok(Key::Esc) => playlist.stop()?,
            Ok(Key::Char(c)) if c == ' ' => playlist.play_pause()?,
            Ok(Key::Ctrl(c)) if c == 'c' => break,
            _ => (),
        }
    }

    Ok(())
});
