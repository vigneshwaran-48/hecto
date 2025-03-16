use std::io::{Write, stdout};

use crossterm::{
    Command,
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{Clear, disable_raw_mode, enable_raw_mode, size},
};

#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cusor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(crossterm::terminal::ClearType::All))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), std::io::Error> {
        Self::queue_command(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cusor_to(position: Position) -> Result<(), std::io::Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        #[allow(clippy::as_conversions)]
        let width = width as usize;
        #[allow(clippy::as_conversions)]
        let height = height as usize;
        Ok(Size { height, width })
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    pub fn print(text: &str) -> Result<(), std::io::Error> {
        Self::queue_command(Print(text))?;
        Ok(())
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
    pub fn queue_command<T: Command>(command: T) -> Result<(), std::io::Error> {
        queue!(stdout(), command)?;
        Ok(())
    }
}
