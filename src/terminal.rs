use std::io::{stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    event::{read, Event, KeyCode, KeyEventKind},
    execute,
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

impl Default for Terminal {
    fn default() -> Self {
        let size = terminal::size();
        terminal::enable_raw_mode().ok();

        match size {
            Ok(res) => Self {
                size: Size {
                    width: res.0,
                    height: res.1.saturating_sub(2),
                },
            },
            Err(_res) => Self {
                size: Size {
                    width: 16,
                    height: 20,
                },
            },
        }
    }
}

impl Terminal {
    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    #[must_use]
    pub fn read_key() -> KeyCode {
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

    fn execute_action(action: impl crossterm::Command) {
        if let Err(err) = execute!(std::io::stdout(), action) {
            eprintln!("Error al ejecutar la acción: {}", err);
        }
    }

    pub fn clear_screen() {
        Self::execute_action(Clear(ClearType::All));
    }

    #[allow(clippy::as_conversions)]
    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x: u16 = *x as u16;
        let y: u16 = *y as u16;

        Self::execute_action(MoveTo(x, y));
    }

    pub fn cursor_hide() {
        Self::execute_action(cursor::Hide);
    }

    pub fn cursor_show() {
        Self::execute_action(cursor::Show);
    }

    pub fn clear_current_line() {
        Self::execute_action(Clear(ClearType::CurrentLine));
    }

    pub fn set_bg_color(color: Color) {
        Self::execute_action(SetBackgroundColor(color));
    }

    pub fn reset_bg_color() {
        Self::execute_action(ResetColor);
    }

    pub fn set_fg_color(color: Color) {
        Self::execute_action(SetForegroundColor(color));
    }

    pub fn reset_fg_color() {
        Self::execute_action(ResetColor);
    }
}
