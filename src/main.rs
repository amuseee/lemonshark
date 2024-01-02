mod editor;
mod terminal; 
use editor::Editor;
pub use terminal::Terminal;
pub use editor::pos;

fn main() {
   let mut lemonshark = Editor::default();
   lemonshark.run();
}
