use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, style::Print};
use std::io::{stdout, Error, Write};

pub struct Terminal;

#[derive(Copy, Clone)]
pub struct Size {
    pub height: u16,
    pub width: u16,
}

#[derive(Copy, Clone)]
pub struct Position{
    pub x: u16,
    pub y:u16,
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode().unwrap();
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0, y:0})?;
        Self.execute()?;
    }

    pub fn execute()->Result<(), Error>{
        stdout().flush()?;

        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();

        queue!(stdout, Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        let mut stdout = stdout();

        queue!(stdout, Clear(ClearType::CurrentLine))
    }


    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }

    pub fn move_cursor_to(position:Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;

        Ok(())
    }

    pub fn hide_cursor() ->Result<(), Error>{
        queue!(stdout(), Hide)?;

        Ok(())
    }

    pub fn show_cursor() -> Result<() Error>{
        queue!(stdout(), Show)?;

        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size();

        Ok(Size {height, width})
    }

    pub fn print(string: &str) -> Result((), Error){
        queue!(stdout(), Print(string))?;

        Ok(())
    }
}
