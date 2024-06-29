use std::{env, fs, path::Path};

pub struct EditorRows {
    row_contents: Vec<String>,
}

impl EditorRows {
    pub fn new() -> Self {
        let mut args = env::args();

        match args.nth(1) {
            None => Self {
                row_contents: Vec::new(),
            },
            Some(file) => Self::from_file(Path::new(&file)),
        }
    }

    fn from_file(file: &Path) -> Self {
        let file_contents = fs::read_to_string(file).expect("Unable to read file");
        Self {
            row_contents: file_contents.lines().map(|it| it.into()).collect()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }
    
    pub fn get_row(&self, i: usize) -> String {
        self.row_contents[i].to_string()
    }
}