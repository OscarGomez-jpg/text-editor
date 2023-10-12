use crossterm::{event::KeyCode, style::Color};
use std::{
    env,
    io::{self},
    time::Instant,
};

use core::time::Duration;

use crate::{terminal::Terminal, Document, Row};

// Definition of two constants named STATUS_BG_COLOR and STATUS_FG_COLOR,
// representing background and foreground colors.
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};

// VERSION constant stores a reference to a string that holds the version information
// obtained from the environment variable CARGO_PKG_VERSION. This likely represents the
// version number of a Rust package or application.
const VERSION: &str = env!("CARGO_PKG_VERSION");

// QUIT_TIMES constant is assigned the value 3 and represents a limit on the number
// of allowed quit times. This value is an unsigned 8-bit integer (u8).
const QUIT_TIMES: u8 = 3;

/// An enum representing the search direction.
///
/// This enum is used to indicate the direction of a search operation,
/// and it can have two possible values: `Forward` and `Backward`.
///
/// - `Forward`: Represents a forward search direction.
/// - `Backward`: Represents a backward search direction.
///
/// # Examples
///
/// ```
/// use my_module::SearchDirection;
///
/// let direction = SearchDirection::Forward;
/// assert_eq!(direction, SearchDirection::Forward);
///
/// let opposite_direction = SearchDirection::Backward;
/// assert_eq!(opposite_direction, SearchDirection::Backward);
/// ```
#[derive(PartialEq, Clone, Copy)]
pub enum SearchDirection {
    Forward,
    Backward,
}

/// A struct representing a 2D position.
///
/// This struct holds the X and Y coordinates of a point in a 2D space.
///
/// # Fields
///
/// - `x`: The X coordinate, represented as a `usize`.
/// - `y`: The Y coordinate, represented as a `usize`.
///
/// # Default
///
/// This struct implements the `Default` trait, allowing you to create instances
/// with default values using `Position::default()`, which sets both `x` and `y` to 0.
///
/// # Clone
///
/// This struct implements the `Clone` trait, allowing you to create cloned copies
/// of `Position` instances.
///
/// # Examples
///
/// ```
/// use my_module::Position;
///
/// let position = Position { x: 10, y: 20 };
/// assert_eq!(position.x, 10);
/// assert_eq!(position.y, 20);
///
/// let default_position = Position::default();
/// assert_eq!(default_position.x, 0);
/// assert_eq!(default_position.y, 0);
///
/// let cloned_position = position.clone();
/// assert_eq!(cloned_position, position);
/// ```
#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

/// A struct representing a status message with text and a timestamp.
///
/// This struct holds a text message and the time it was created, represented
/// as an `Instant` instance.
///
/// # Fields
///
/// - `text`: The text of the status message, represented as a `String`.
/// - `time`: The timestamp when the status message was created, represented as an `Instant`.
///
/// # Examples
///
/// ```
/// use my_module::StatusMessage;
/// use std::time::Instant;
///
/// let timestamp = Instant::now();
/// let message = StatusMessage {
///     text: "Hello, World!".to_string(),
///     time: timestamp,
/// };
///
/// assert_eq!(message.text, "Hello, World!");
/// assert_eq!(message.time, timestamp);
/// ```
///
/// # Creation
///
/// You can create a `StatusMessage` instance from a `String` message using the `from` method.
///
/// ```
/// use my_module::StatusMessage;
///
/// let message = StatusMessage::from("An important message".to_string());
/// ```
struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    /// Creates a new `StatusMessage` instance from a given text message.
    ///
    /// This method takes a `String` message and generates a `StatusMessage` instance
    /// with the provided message text and the current timestamp.
    ///
    /// # Arguments
    ///
    /// - `message`: A `String` containing the text message.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_module::StatusMessage;
    ///
    /// let message = StatusMessage::from("An important message".to_string());
    /// ```
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}

/// A struct representing a text editor.
///
/// This struct encapsulates the state and functionality of a simple text editor.
/// It contains various fields to store information such as the editor's terminal,
/// cursor position, document, and status message, among others.
///
/// # Fields
///
/// - `should_quit`: A boolean flag indicating whether the editor should quit.
/// - `terminal`: An instance of the `Terminal` struct for interacting with the terminal.
/// - `cursor_position`: The current position of the cursor, represented as a `Position`.
/// - `offset`: The offset position, often used for scrolling, represented as a `Position`.
/// - `document`: An instance of the `Document` struct representing the text document.
/// - `status_message`: An instance of the `StatusMessage` struct for displaying status messages.
/// - `quit_times`: An unsigned 8-bit integer (`u8`) representing the number of allowed quit times.
/// - `highlighted_word`: An optional `String` representing a currently highlighted word.
///
/// # Examples
///
/// ```
/// use my_module::Editor;
///
/// // Create a new text editor instance.
/// let mut editor = Editor::new();
///
/// // Initialize the editor's state and interact with it.
/// editor.open_document("example.txt");
/// editor.insert_text("Hello, World!");
/// editor.move_cursor(8, 0);
/// editor.delete_word();
/// editor.save_document();
/// ```
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
    highlighted_word: Option<String>,
}

impl Editor {
    //Constructor
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: F3 = find | F5 = save | F8 = quit");

        //Opening a file, otherwise, main application
        let document = if let Some(file_name) = args.get(1) {
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {file_name}");
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::default(),
            document,
            cursor_position: Position::default(),
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
            highlighted_word: None,
        }
    }

    //Callable implementation
    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(&error);
            }

            if self.should_quit {
                crossterm::terminal::disable_raw_mode().ok();
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    //Private keyboard processor
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let actual_key: KeyCode = Terminal::read_key();

        match actual_key {
            KeyCode::F(8) => {
                if self.quit_times > 0 && self.document.is_dirty() {
                    self.status_message = StatusMessage::from(format!(
                        "WARNING! File has unsaved changes. Press F8 {} more times to quit.",
                        self.quit_times
                    ));
                    self.quit_times -= 1;
                    return Ok(());
                }
                self.should_quit = true;
            }
            KeyCode::F(3) => self.search(),
            KeyCode::F(5) => self.save(),
            KeyCode::Enter => {
                self.document.insert(&self.cursor_position, '\n');
                self.move_cursor(KeyCode::Right);
            }
            KeyCode::Tab => {
                self.document.insert(&self.cursor_position, '\t');
                self.move_cursor(KeyCode::Tab);
            }
            KeyCode::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(KeyCode::Right);
            }
            KeyCode::Delete => self.document.delete(&self.cursor_position),
            KeyCode::Backspace => {
                if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                    self.move_cursor(KeyCode::Left);
                    self.document.delete(&self.cursor_position);
                }
            }
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

        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
        //This is used to propagate the error along the system
        Ok(())
    }

    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt("Search: ", |editor, key_code, query| {
                let mut moved = false;
                match key_code {
                    KeyCode::Right | KeyCode::Down => {
                        direction = SearchDirection::Forward;
                        editor.move_cursor(KeyCode::Right);
                        moved = true;
                    }
                    KeyCode::Left | KeyCode::Up => direction = SearchDirection::Backward,
                    _ => direction = SearchDirection::Forward,
                }

                if let Some(position) =
                    editor
                        .document
                        .find(&query, &editor.cursor_position, direction)
                {
                    editor.cursor_position = position;
                    editor.scroll();
                } else if moved {
                    editor.move_cursor(KeyCode::Left);
                }

                editor.highlighted_word = Some(query.to_string());
            })
            .unwrap_or(None);

        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }

        self.highlighted_word = None;
    }

    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, KeyCode, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{prompt}{result}"));
            self.refresh_screen()?;
            let key = Terminal::read_key();
            match key {
                KeyCode::Backspace => result.truncate(result.len().saturating_sub(1)),
                KeyCode::Enter => break,
                KeyCode::Char(c) => {
                    if !c.is_control() {
                        result.push(c);
                    }
                }
                KeyCode::Esc => {
                    result.truncate(0);
                    break;
                }
                _ => (),
            }
            callback(self, key, &result)
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width: usize = self.terminal.size().width.try_into().unwrap_or_default();
        let height: usize = self.terminal.size().height.try_into().unwrap_or_default();
        let offset = &mut self.offset;

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
        let terminal_height = self.terminal.size().height.try_into().unwrap_or_default();
        let Position { mut y, mut x } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        match key_selection {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y.saturating_sub(terminal_height)
                } else {
                    0
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y.saturating_add(terminal_height)
                } else {
                    height
                }
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            KeyCode::Tab => {
                if x < width {
                    x += 4;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            _ => (),
        }

        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };

        if x > width {
            x = width;
        }

        self.cursor_position = Position { x, y }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();

        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye.\r");
        } else {
            self.document.highlight(
                &self.highlighted_word,
                Some(
                    self.offset
                        .y
                        .saturating_add(self.terminal.size().height.try_into().unwrap_or_default()),
                ),
            );

            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            });
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn save(&mut self) {
        if self.document.get_file_name().is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);

            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted: ".to_string());
                return;
            }

            self.document.set_file_name(new_name);
        }

        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writing file!".to_string());
        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Voider -- version {VERSION}");
        let width: usize = self.terminal.size().width.try_into().unwrap_or_default();
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);

        println!("{welcome_message}\r");
    }

    fn draw_row(&self, row: &Row) {
        let width: usize = self.terminal.size().width.try_into().unwrap_or_default();
        let start = self.offset.x;
        let end = self.offset.x.saturating_add(width);
        let row = row.render(start, end);

        println!("{row}\r");
    }

    fn draw_rows(&mut self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(
                self.offset
                    .y
                    .saturating_add(terminal_row.try_into().unwrap_or_default()),
            ) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_status_bar(&self) {
        let mut status;
        let width: usize = self.terminal.size().width.try_into().unwrap_or_default();

        let modifier_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };

        let mut file_name = "[No Name]".to_string();

        if let Some(name) = &self.document.get_file_name() {
            file_name = name.clone();
            file_name.truncate(20);
        }

        status = format!(
            "{} - {} lines {}",
            file_name,
            self.document.len(),
            modifier_indicator
        );

        let line_indicator = format!(
            "{} | {} / {}",
            self.document.file_type(),
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );

        #[allow(clippy::arithmetic_side_effects)]
        let len = status.len() + line_indicator.len();
        status.push_str(&" ".repeat(width.saturating_sub(len)));
        status = format!("{status}{line_indicator}");
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if message.time.elapsed() < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width.try_into().unwrap_or_default());
            print!("{text}");
        }
    }
}

//Error catcher
fn die(e: &io::Error) {
    panic!("{e:?}");
}
