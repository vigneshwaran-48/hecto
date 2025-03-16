use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};

mod terminal;
use terminal::{Position, Terminal};

mod view;
use view::View;

#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    is_welcome_screen: bool,
    cursor_position: Position,
    view: View,
}

impl Editor {
    pub fn run(&mut self, file_path: Option<&str>) {
        Terminal::initialize().unwrap();
        if let Some(file_path) = file_path {
            self.view.load(file_path).unwrap();
        }
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
            self.view.render()?;
            Terminal::move_cusor_to(Position {
                x: self.cursor_position.x,
                y: self.cursor_position.y,
            })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
}
