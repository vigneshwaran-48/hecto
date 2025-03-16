use crossterm::event::{Event, KeyEvent, KeyModifiers};
use crossterm::event::{Event::Key, KeyCode::Char, read};

mod terminal;
use terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
    is_welcome_screen: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_quit: false,
            is_welcome_screen: true,
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
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => self.should_quit = true,
                _ => (),
            }
        }
        if self.is_welcome_screen {
            self.is_welcome_screen = false;
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::move_cusor_to(Position { x: 0, y: 0 })?; // After the welcome message the
            // width is not cleared properly. This will reset the width.
            Terminal::print("Goodbye \r\n")?;
        } else if self.is_welcome_screen {
            Self::draw_rows()?;
            Self::display_welcome_message()?;
        } else {
            Terminal::move_cusor_to(Position { x: 0, y: 0 })?; // After the welcome message the
            // width is not cleared properly. This will reset the width.
            Self::draw_rows()?;
            Terminal::move_cusor_to(Position { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn display_welcome_message() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;

        let message = "Hecto";

        let y = size.height / 3;
        let x = (size.width - (message.len() as u16)) / 2;

        Terminal::move_cusor_to(Position { x, y })?;
        Terminal::print(message)?;

        let message = "1.0";
        let y = y + 1;

        let x = (size.width - (message.len() as u16)) / 2;

        Terminal::move_cusor_to(Position { x, y })?;
        Terminal::print(message)?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_height in 0..height {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if current_height + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
