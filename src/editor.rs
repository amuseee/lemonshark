use std::io::{stdin, stdout, Error};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(&self) {
        let _out = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                diew(error);
            }
        }
    }

    pub fn default() -> Self {
        Self{}
    }

    fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
    fn process_keypress(&self) -> Result<(), Error> {
        let keypress = Self::read_key()?;
        if let Key::Ctrl('q') = keypress {
            panic!("exitting lemonshark...");
        }
        Ok(())
    }
}

fn diew(e: Error) {
    panic!("{}", e);
}