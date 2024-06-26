use crate::*;

pub struct Reader;

impl Reader {
    pub fn read_keypress(&self) -> Result<KeyEvent, Error> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}
