use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    queue,
    terminal::{self, Clear, ClearType},
};

use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = terminal::size()?;
        terminal::enable_raw_mode().ok();
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        (queue!(stdout(), Clear(ClearType::All))).unwrap();
    }

    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = *x as u16;
        let y = *y as u16;
        (queue!(stdout(), cursor::MoveTo(x, y))).unwrap();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn read_key() -> KeyCode {
        loop {
            match read() {
                Ok(Event::Key(event)) => {
                    return event.code;
                }
                Err(err) => panic!("{err:?}"),
                _ => (),
            }
        }
    }

    pub fn cursor_hide() {
        (queue!(stdout(), cursor::Hide)).unwrap();
    }

    pub fn cursor_show() {
        (queue!(stdout(), cursor::Show)).unwrap();
    }

    pub fn clear_current_line() {
        (queue!(stdout(), Clear(ClearType::CurrentLine))).unwrap();
    }
}
