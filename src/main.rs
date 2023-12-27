use std::io::{stdin, stdout, Error};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn diew(e: Error) {
    panic!("{}", e);
}

fn main() {
    let _out = stdout().into_raw_mode().unwrap();

    for key in stdin().keys() {
        match key {
            Ok(key) => match key {
                Key::Char(c) => {
                    if c.is_control() {
                        println!("{:?}\r", c as u8);
                    } else {
                        println!("{:?} ({})\r", c as u8, c);
                    }
                }
                Key::Ctrl('q') => break,
                _ => println!("{:?}\r", key),
            },
            Err(err) => diew(err),
        }
    }
}
