use std::io::{Write, stdout};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{Clear, disable_raw_mode, enable_raw_mode, size},
};

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cusor_to(0, 0)?;
        stdout().flush()
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(crossterm::terminal::ClearType::All))?;
        Ok(())
    }
    pub fn move_cusor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)
    }
    pub fn print(text: &str) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(text))
    }
}
