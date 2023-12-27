use std::io::{self, stdin, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {

    let _stdout = stdout().into_raw_mode().unwrap();

    for i in stdin().bytes() { 
        let i = i.unwrap();
        let c = i as char;
        if c.is_control() {
            println!("{} \r", i);
        } else {
            println!("{} ({})\r", i, c);
        }
        if c == 'q' {
            break;
        }
    }
}
