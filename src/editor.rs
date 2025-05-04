use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;

use terminal::Terminal;

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
        let height = Terminal::size()?.1;

        for c_row in 0..height {
            print!("~");

            if c_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye! \r");
        } else {
            Self::draw_tildes()?;
            Terminal::move_cursor_to(0, 0)?;
        }

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
