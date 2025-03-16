use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    execute,
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
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(crossterm::terminal::ClearType::All))?;
        Ok(())
    }
    pub fn move_cusor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(x, y))?;
        Ok(())
    }
    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }
}
