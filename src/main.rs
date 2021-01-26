#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod row;
mod document;
pub use editor::Position;
pub use terminal::Terminal;
pub use document::Document;
pub use row::Row;
use editor::Editor;

fn main() {
    Editor::default().run();
}