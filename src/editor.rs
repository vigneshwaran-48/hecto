use std::env;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};

mod terminal;
use terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
    is_welcome_screen: bool,
    cursor_position: Position,
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            is_welcome_screen: true,
            cursor_position: Position { x: 0, y: 0 },
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    pub fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press, // For windows
            ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                KeyCode::Left => {
                    let terminal_pos = Terminal::size().unwrap();
                    if self.cursor_position.x.saturating_sub(1) > 0 {
                        self.cursor_position.x = self.cursor_position.x.saturating_sub(1);
                    } else {
                        self.cursor_position.x = terminal_pos.width;
                        self.cursor_position.y = self.cursor_position.y.saturating_sub(1);
                    }
                }
                KeyCode::Right => {
                    let terminal_pos = Terminal::size().unwrap();
                    if self.cursor_position.x.saturating_add(1) < terminal_pos.width {
                        self.cursor_position.x = self.cursor_position.x.saturating_add(1);
                    } else {
                        self.cursor_position.x = 0;
                        self.cursor_position.y = self.cursor_position.y.saturating_add(1);
                    }
                }
                KeyCode::Up => {
                    self.cursor_position.y = self.cursor_position.y.saturating_sub(1);
                    // Not interested in providing support for folding in vertical movement.
                }
                KeyCode::Down => {
                    let terminal_pos = Terminal::size().unwrap();
                    if self.cursor_position.y.saturating_add(1) < terminal_pos.height {
                        self.cursor_position.y = self.cursor_position.y.saturating_add(1);
                    }
                    // Not interested in providing support for folding in vertical movement.
                }
                KeyCode::End => {
                    let terminal_pos = Terminal::size().unwrap();
                    self.cursor_position.x = terminal_pos.width;
                }
                KeyCode::Home => {
                    self.cursor_position.x = 0;
                }
                KeyCode::PageUp => {
                    self.cursor_position.y = 0;
                }
                KeyCode::PageDown => {
                    self.cursor_position.y = Terminal::size().unwrap().height;
                }
                _ => (),
            }
        }
        if self.is_welcome_screen {
            self.is_welcome_screen = false;
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::move_cusor_to(Position { x: 0, y: 0 })?;
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye \r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cusor_to(Position {
                x: self.cursor_position.x,
                y: self.cursor_position.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_height in 0..height {
            Terminal::clear_line()?;
            // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
            // it's allowed to be a bit up or down 1Has a conversation. Original line has a conversation.
            #[allow(clippy::integer_division)]
            if current_height == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_height.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }
}
