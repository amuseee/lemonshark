use std::io;
use std::io::stdin;
use std::io::Read;


fn main() {
    for b in stdin().bytes() {
        let c = b.unwrap() as char;
        print!("{}", c);
    }
}
