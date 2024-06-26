use crate::*;

pub struct Output {
    pub win_size: (usize, usize),
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
        .map(|(x, y)| (x as usize, y as usize))
        .unwrap();
        
        Self {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
        }
    }

    pub fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn refresh_screen(&mut self) -> Result<(), Error> {
        queue!(
            self.editor_contents,
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        self.draw_rows();

        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;

        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        
        self.editor_contents.flush()
    }

    pub fn draw_rows(&mut self) {
        let num_columns = self.win_size.0;
        let num_rows = self.win_size.1;
        for i in 0..num_rows {
            if i == num_rows / 3 {
                let mut welcome = format!("Text Editor\r");
                if welcome.len() > num_columns {
                    welcome.truncate(num_columns);
                }
                let mut padding = (num_columns - welcome.len())/2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome);

            } else {
                self.editor_contents.push_str("~\r\n");
                queue!(
                    self.editor_contents,
                    terminal::Clear(ClearType::CurrentLine)
                ).unwrap()
            }
        }
        self.editor_contents.push('~');
        self.editor_contents.flush().unwrap();
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

}