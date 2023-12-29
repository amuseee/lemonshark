use std::io::Stdout;
use std::io::{stdin, stdout, Error, Write};
// I WILL!! reimplement cross platform support!
use termion::clear;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{self, IntoRawMode};
use termion::cursor;


pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _out: raw::RawTerminal<Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _out: stdout().into_raw_mode()?,
        })
    }
    pub fn size(&self) -> &Size {
        &self.size
    }
    pub fn scr_clear() {
        print!("{}", clear::All);
    }
    pub fn cursorpos(x: u16, y: u16) {
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", cursor::Goto(x, y));
    }
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }
    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
}