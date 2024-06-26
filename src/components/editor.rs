use crate::*;

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {reader: Reader, output: Output::new()}
    }

    pub fn process_keypress(&mut self) -> Result<bool, Error> {
        let event = self.reader.read_keypress()?;
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => return Ok(false),
            KeyEvent {
                code: key @ (
                    KeyCode::Char('h')
                    | KeyCode::Char('j')
                    | KeyCode::Char('k')
                    | KeyCode::Char('l')
                    | KeyCode::Home
                    | KeyCode::End
                ),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => {
                self.output.move_cursor(key);
            },
            KeyEvent {
                code: key @ (
                    KeyCode::PageUp
                    | KeyCode::PageDown
                ),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => {
                (0..self.output.win_size.1).for_each(|_| {
                    self.output.move_cursor(if key == KeyCode::PageUp {
                        KeyCode::Char('k')
                    } else {
                        KeyCode::Char('j')
                    });
                })
            },
            _ => {}
        }
        return Ok(true)
    }

    pub fn run(&mut self) -> Result<bool, Error> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}