use std::io;
use std::io::Read;


fn main() {
    for i in io::stdin().bytes() {
        let c = i.unwrap() as char;
        print!("{}", c);
        if c == 'q' || c == 'Q' {
            break;
        }
    }
}
