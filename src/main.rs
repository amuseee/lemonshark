use std::io::{self, stdin, stdout, Read};
use termion::raw::IntoRawMode;

fn main() {
    for i in stdin().bytes() {
        
        let _stdout = stdout().into_raw_mode().unwrap();
        
        let c = i.unwrap() as char;
        print!("{}", c);
        if c == 'q' || c == 'Q' {
            break;
        }
    }
}
