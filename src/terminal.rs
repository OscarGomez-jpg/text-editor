use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEventKind},
    queue,
    style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor},
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
                height: size.1.saturating_sub(2),
            },
        })
    }

    #[must_use] pub fn size(&self) -> &Size {
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

    #[must_use] pub fn read_key() -> KeyCode {
        loop {
            match read() {
                Ok(Event::Key(event)) => {
                    //This is to make sure that crossterm will only read when the key is pressed
                    if let KeyEventKind::Press = event.kind {
                        return event.code;
                    }
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

    pub fn set_bg_color(color: Color) {
        (queue!(stdout(), SetBackgroundColor(color))).unwrap();
    }

    pub fn reset_bg_color() {
        (queue!(stdout(), ResetColor)).unwrap();
    }

    pub fn set_fg_color(color: Color) {
        (queue!(stdout(), SetForegroundColor(color))).unwrap();
    }

    pub fn reset_fg_color() {
        (queue!(stdout(), ResetColor)).unwrap() //I now that is repeated, but I just want to follow the logic
    }
}
