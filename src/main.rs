use std::io::{self, stdin, stdout, Read, Error};
use termion::raw::IntoRawMode;


// TODO refactor termion w/ crossterm to add windows support (ugh)

fn errorprint(e: Error) {
    panic!("{}", e);
}


fn ctrl_and(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b00011111 // bitwise magic!!
}

fn main() {

    let _stdout = stdout().into_raw_mode().unwrap();

    for i in stdin().bytes() { 

        match i {
            Ok(i) => {
                let c = i as char;
                if c.is_control() {
                    println!("{:?} \r", i);
                } else {
                    println!("{:?} ({})\r", i, c);
                }

                if i == ctrl_and('q') {
                    break;
                 }
            }
            Err(err) => {
                errorprint(err)
            }
            
        }
    }
}
