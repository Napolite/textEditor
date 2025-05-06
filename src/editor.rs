use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;

use terminal::{Terminal, Size, Position};


 const NAME: &str = env!("CARGO_PKG_NAME");
 const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();

        let result = self.repl();

        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn draw_tildes() -> Result<(), std::io::Error> {
        let Size{height, ..}= Terminal::size()?;

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
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye! \r")?;
        } else {
            Self::draw_tildes()?;    
            Terminal::move_cursor_to(Position{x:0, y:0})?;
    }
        Terminal::show_cursor()?;
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
