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
mod track;

use config::Config;
use errors::*;
use std::env::args;
use std::fs::File;
use std::io::{Read, stdin};
use std::process::exit;
use termion::event::Key;
use termion::input::TermRead;
use track::Tracks;

quick_main!(|| -> Result<()> {
    let mut args = args();
    let path = match args.nth(1) {
        Some(p) => p,
        None => {
            println!("Usage: ontrack <config_path>");
            exit(1);
        },
    };

    // Load config
    let mut toml = String::new();
    let mut fh = File::open(&path)?;
    fh.read_to_string(&mut toml)?;
    let config: Config = toml::from_str(&toml)?;

    // Load Tracks
    let mut tracks = Tracks::from_config(config)?;
    tracks.announce_track();

    let stdin = stdin();
    for c in stdin.keys() {
        match c {
            Ok(Key::Left) => tracks.previous()?,
            Ok(Key::Right) => tracks.next()?,
            Ok(Key::Esc) => tracks.stop()?,
            Ok(Key::Char(c)) if c == ' ' => tracks.play_pause()?,
            Ok(Key::Ctrl(c)) if c == 'c' => break,
            _ => (),
        }
    }

    Ok(())
});
