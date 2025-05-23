use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, style::Print, Command};
use std::io::{stdout, Error, Write};
use core::fmt::Display;


#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone, Default)]
pub struct Position{
    pub col: usize,
    pub row:usize,
}

pub struct Terminal;


impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode().unwrap();
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn execute()->Result<(), Error>{
        stdout().flush()?;

        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();

        Self::queue(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        let mut stdout = stdout();

        Self::queue(Clear(ClearType::CurrentLine))
    }


    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn move_caret_to(position:Position) -> Result<(), Error> {
        Self::queue(MoveTo(position.col as u16, position.row as u16))?;

        Ok(())
    }

    pub fn hide_caret() ->Result<(), Error>{
        Self::queue(Hide)?;

        Ok(())
    }

    pub fn show_caret() -> Result<(), Error>{
        Self::queue(Show)?;

        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;

        Ok(Size {height, width})
    }

    pub fn print<T:Display>(string: T) -> Result<(), Error>{
        Self::queue(Print(string))?;

        Ok(())
    }

    pub fn queue<T:Command>(command:T) -> Result<(), Error>{
        queue!(stdout(), command);

        Ok(())
    }
}
