use core::cmp::min;
use crossterm::event::{
    Event,
    Event::{Key},
    KeyCode::{self, Char},
    KeyEvent,KeyEventKind, KeyModifiers, read,
};
use std::default;
mod terminal;

use terminal::{Position, Size, Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]

struct Location {
    x: usize,
    y: usize,
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    location: Location,
}

impl Editor {
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();

        let result = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), std::io::Error> {
        if let Key(KeyEvent {
            code, modifiers, kind:KeyEventKind::Press,..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                 KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home =>{
                    self.move_point(*code)?; 
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn draw_tildes() -> Result<(), std::io::Error> {
        let Size { height, .. } = Terminal::size()?;

        for c_row in 0..height {
            Terminal::clear_line()?;
            if c_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if c_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! \r")?;
        } else {
            Self::draw_tildes()?;
            Terminal::move_caret_to(Position { col: self.location.x, row: self.location.y })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;

        Ok(())
    }

    fn draw_welcome_message() -> Result<(), std::io::Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)?;
        Ok(())
    }
    fn draw_empty_row() -> Result<(), std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), std::io::Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap();

        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1) as usize;
            }
            KeyCode::Down =>{ 
                y = min(height.saturating_sub(1) as usize, y.saturating_add(1)) as usize;
            }
            KeyCode::Left =>{ 
                x = x.saturating_sub(1) as usize;
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1) as usize, x.saturating_add(1)) as usize;
            }
            KeyCode::PageUp=>{
                y = 0;
            }
            KeyCode::PageUp => {
                y = height.saturating_sub(1) as usize;
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1) as usize;
            }
            _ => (),
        }
        self.location = Location{x,y};
        Ok(())
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
}
