use crate::Terminal;
use std::io::Error;
// i WILL! reimplement this with cross platform support!!
use termion::event::Key;

pub struct Editor {
    go_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) {
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
        Self {
            go_quit: false, 
            terminal: Terminal::default().expect("failed to init terminal"),
        }
    }
    fn scr_refresh(&self) -> Result<(), Error> {
        Terminal::scr_clear();
        Terminal::cursorpos(0, 0);
        if self.go_quit {
            println!("shork hopes you'll be back c:\r");
        } else {
            self.draw_rows();
            Terminal::cursorpos(0, 0);
        }
        Terminal::flush()
    }
    fn draw_rows (&self) {
        for _ in 0..24 {
            println!("~\r");
        }
    } 
    fn process_keypress(&mut self) -> Result<(), Error> {
        let keypress = Terminal::read_key()?;
        if let Key::Ctrl('q') = keypress {
            self.go_quit = true;
        }
        Ok(())
    }
}

fn diew(e: Error) {
    Terminal::scr_clear();
    panic!("{}", e);
}