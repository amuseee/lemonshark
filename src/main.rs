use std::io::{self, stdin, stdout, Read, Error};
use crossterm::terminal;

fn errorprint(e: Error) {
    panic!("{}", e);
}


fn ctrl_and(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b00011111 // bitwise magic!!
}

fn main() {

    let _stdout = terminal::enable_raw_mode();

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
                    let _stdout = terminal::disable_raw_mode();
                    break;
                 }
            }
            Err(err) => {
                errorprint(err)
            }
            
        }
    }
}
