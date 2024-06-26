use crate::*;

pub struct EditorContents {
    content: String,
}

impl EditorContents {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn push(&mut self, ch: char) {
        self.content.push(ch);
    }

    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str);
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(v) => {
                self.content.push_str(v);
                Ok(v.len())
            },
            Err(_) => Err(io::ErrorKind::WriteZero.into())
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}