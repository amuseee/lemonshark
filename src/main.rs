mod editor;
mod terminal; 
use editor::Editor;
pub use terminal::Terminal;

fn main() {
   let mut lemonshark = Editor::default();
   lemonshark.run();
}
