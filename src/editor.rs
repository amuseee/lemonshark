use crate::Terminal;
use std::io::Error;
use termion::event::Key;

pub struct pos {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    go_quit: bool,
    terminal: Terminal,
    cursor_pos: pos,
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
            cursor_pos: pos{x: 0, y: 0},
        }
    }
    fn scr_refresh(&self) -> Result<(), Error> {
        Terminal::hide_cursor();
        Terminal::cursorpos(&pos{x: 0, y: 0});
        if self.go_quit {
            Terminal::scr_clear();
            println!("shork hopes you'll be back c:\r");
        } else {
            self.draw_rows();
            Terminal::cursorpos(&self.cursor_pos);
        }
        Terminal::show_cursor();
        Terminal::flush()
    }
    fn welcome(&self) {
        /* TODO implement ascii art on welcome screen
        let shark = r#" 
             .';
         .-'` .'
       ,`.-'-.`\
      ; /     '-'
      | \       ,-,
      \  '-.__   )_`'._
       '.     ```      ``'--._
      .-' ,                   `'-.
       '-'`-._           ((   o   )
             `'--....(`- ,__..--'
                       '-'`
        "#; */
        let mut message = format!("lemonshark text editor");
        let width = self.terminal.size().width as usize;
        let len = message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{}{}", spaces, message);
        message.truncate(width);
        println!("{}\r", message);
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