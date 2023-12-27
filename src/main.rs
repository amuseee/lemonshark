use std::io::{self, stdin, stdout, Read};


fn main() {
    for i in stdin().bytes() {
        let c = i.unwrap() as char;
        print!("{}", c);
        if c == 'q' || c == 'Q' {
            break;
        }
    }
}
