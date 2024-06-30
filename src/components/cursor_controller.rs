use crate::*;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    num_columns: usize,
    num_rows: usize,
    pub row_offset: usize,
    pub column_offset: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            num_columns: win_size.0,
            num_rows: win_size.1,
            row_offset: 0,
            column_offset: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        match direction {
            KeyCode::Char('k') => {
                if self.cursor_y != 0 {
                    self.cursor_y -= 1;   
                }
            },
            KeyCode::Char('h') => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = editor_rows.get_row(self.cursor_y).len();
                }
            },
            KeyCode::Char('j') => {
                if self.cursor_y < editor_rows.number_of_rows() {
                    self.cursor_y += 1;
                }
            },
            KeyCode::Char('l') => {
                if self.cursor_y < editor_rows.number_of_rows() && self.cursor_x < editor_rows.get_row(self.cursor_y).len() {
                    self.cursor_x += 1;
                } else if self.cursor_y < editor_rows.number_of_rows() {
                    self.cursor_y += 1;
                    self.cursor_x = 0;
                }
            },
            KeyCode::Home => {
                self.cursor_x = 0;
            },
            KeyCode::End => {
                self.cursor_x = self.num_columns - 1
            }
            _ => unimplemented!(),
        }
        let row_len = if self.cursor_y < editor_rows.number_of_rows() {
            editor_rows.get_row(self.cursor_y).len()
        } else {
            0
        };

        if row_len < self.cursor_x {
            self.cursor_x = row_len
        }
    }

    pub fn scroll(&mut self) {
        // self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        }

        let mut file = std::fs::File::options().append(true).open("offset_log.txt").unwrap();
        file.write(format!("{} {}\r\n", self.row_offset, self.cursor_y).as_bytes()).unwrap();

        if self.cursor_y >= self.row_offset + self.num_rows {
            self.row_offset = self.cursor_y - self.num_rows + 1;
        }

        if self.cursor_x < self.column_offset {
            self.column_offset = self.cursor_x
        }
        if self.cursor_x >= self.column_offset + self.num_columns {
            self.column_offset = self.cursor_x - self.num_columns + 1
        }
    }

}