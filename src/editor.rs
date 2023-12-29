use crate::Terminal;
use std::io::Error;
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
            if self.go_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                diew(error);
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
        Terminal::hide_cursor();
        Terminal::cursorpos(0, 0);
        if self.go_quit {
            Terminal::scr_clear();
            println!("shork hopes you'll be back c:\r");
        } else {
            self.draw_rows();
            Terminal::cursorpos(0, 0);
        }
        Terminal::show_cursor();
        Terminal::flush()
    }
    fn welcome(&self) {
        let mut message = format!("lemonshark text editor");
        let mut message2 = format!("shork loves chomping on text ><>");
        let width = self.terminal.size().width as usize;
        let len = message.len();
        let len2 = message2.len();
        let padding = width.saturating_sub(len) / 2;
        let padding2 = width.saturating_sub(len2) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        let spaces2 = " ".repeat(padding2.saturating_sub(1));
        message = format!("~{}{}", spaces, message);
        message2 = format!("~{}{}", spaces2, message2);
        message.truncate(width);
        message2.truncate(width);
        println!("{}\r", message);
        println!("{}\r", message2);
    }
    fn draw_rows (&self) {
        let height = self.terminal.size().height;
        for row in 0..height -1 {
            Terminal::clear_line();
            if row == height / 3 {
                self.welcome();
            } else {
                println!("~\r");
            }
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