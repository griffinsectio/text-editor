use crate::*;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Can't disable raw mode!");
        Output::clear_screen().expect("Can't clear screen!");
    }
}
