use std::io::{stdin, stdout, Error, Write};
// i WILL! reimplement this with cross platform support!!
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::clear;
use termion::cursor;

pub struct Editor {
    go_quit: bool,
}

impl Editor {
    pub fn run(&mut self) {
        let _out = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.scr_refresh() {
                diew(error);
            }
            if let Err(error) = self.process_keypress() {
                diew(error);
            }
            if self.go_quit {
                break;
            }
        }
    }
    pub fn default() -> Self {
        Self{go_quit: false}
    }
    fn scr_refresh(&self) -> Result<(), Error> {
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        if self.go_quit {
            println!("shork hopes you'll be back c:\r");
        }
        stdout().flush()
    } 
    fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }
    fn process_keypress(&mut self) -> Result<(), Error> {
        let keypress = Self::read_key()?;
        if let Key::Ctrl('q') = keypress {
            self.go_quit = true;
        }
        Ok(())
    }
}

fn diew(e: Error) {
    print!("{}", clear::All);
    panic!("{}", e);
}