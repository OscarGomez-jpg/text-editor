use std::io::{stdout, Stdout, Write};

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
    stdout: Stdout,
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
            stdout: stdout(),
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen(&mut self) {
        let _result = queue!(self.stdout, Clear(ClearType::All));
    }

    pub fn cursor_position(&mut self, position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(0);
        y = y.saturating_add(0);
        let x = x as u16;
        let y = y as u16;
        let _result = queue!(self.stdout, cursor::MoveTo(x, y));
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.stdout.flush()
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

    pub fn cursor_hide(&mut self) {
        let _result = queue!(self.stdout, cursor::Hide);
    }

    pub fn cursor_show(&mut self) {
        let _result = queue!(self.stdout, cursor::Show);
    }

    pub fn clear_current_line(&mut self) {
        let _result = queue!(self.stdout, Clear(ClearType::CurrentLine));
    }
}
