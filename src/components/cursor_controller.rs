use crate::*;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    num_columns: usize,
    num_rows: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            num_columns: win_size.0,
            num_rows: win_size.1,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Char('k') => {
                if self.cursor_y != 0 {
                    self.cursor_y -= 1;   
                }
            },
            KeyCode::Char('h') => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                }
            },
            KeyCode::Char('j') => {
                if self.cursor_y < self.num_rows - 1 {
                    self.cursor_y += 1;
                }
            },
            KeyCode::Char('l') => {
                if self.cursor_x < self.num_columns - 1 {
                    self.cursor_x += 1;
                }
            },
            KeyCode::Home => {
                self.cursor_x = 0;
            },
            KeyCode::End => {
                self.cursor_x = self.num_columns - 1
            }
            _ => unimplemented!()
        }
    }

}