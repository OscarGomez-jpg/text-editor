use crossterm::{cursor, event::KeyCode, queue};
use std::{
    env,
    io::{self, stdout},
};

use crate::{terminal::Terminal, Document, Row};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
}

impl Editor {
    //Constructor
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        //Opening a file, otherwise, main application
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Jesus Christ, what have you done?"),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
        }
    }

    //Callable implementation
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        let _result = queue!(stdout(), cursor::Hide);
        Terminal::cursor_position(&mut self.terminal, &Position::default());
        if self.should_quit {
            Terminal::clear_screen(&mut self.terminal);
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&mut self.terminal, &self.cursor_position);
        }
        Terminal::cursor_show(&mut self.terminal);
        Terminal::flush(&mut self.terminal)
    }

    //Private keyboard processor
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let actual_key: KeyCode = Terminal::read_key();
        match actual_key {
            KeyCode::F(8) => self.should_quit = true,
            KeyCode::Up
            | KeyCode::Down
            | KeyCode::Left
            | KeyCode::Right
            | KeyCode::PageUp
            | KeyCode::PageDown
            | KeyCode::End
            | KeyCode::Home => self.move_cursor(actual_key),
            _ => (),
        }
        self.scroll();
        //This is used to propagate the error along the system
        Ok(())
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn move_cursor(&mut self, key_selection: KeyCode) {
        let Position { mut y, mut x } = self.cursor_position;
        let size = self.terminal.size();
        let height = self.document.len();
        let width = size.width.saturating_sub(1) as usize;
        match key_selection {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Voider -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&mut self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line(&mut self.terminal);
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }
}

//Error catcher
fn die(e: &io::Error) {
    panic!("{e:?}");
}
