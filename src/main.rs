// TODO: structopt or clap or something
use std::env;
pub use std::io;
use std::fs;

extern crate rsimapnotify;

fn main() -> Result<(), io::Error> {
    let path = env::args().nth(1).ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Path required"))?;
    // probably always going to be safe to read the whole file in, it's not _huge_
    let contents = fs::read_to_string(path)?;
    rsimapnotify::config::parser::from_mbsync(contents)
}
