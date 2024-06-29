mod components;
use components::cleanup::*;
use components::cursor_controller::*;
use components::editor_contents::*;
use components::editor::*;
use components::output::*;
use components::reader::*;
use components::editor_rows::*;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::{cursor, event, execute, queue, terminal};
use crossterm::terminal::ClearType;
use std::io::{self, stdout, Error, Write};
use std::time::Duration;

fn main() -> Result<(), Error> {
    let _cleanup = CleanUp;
    print!("\x1b[2J");

    terminal::enable_raw_mode()?;

    let mut editor = Editor::new();
    while editor.run()? {}

    Ok(())
}
