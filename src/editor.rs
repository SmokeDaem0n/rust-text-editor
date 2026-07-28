use crossterm::event::{Event::Key, KeyCode::Char, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Editor {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.place_holder() {
            panic!("{err:#?}");
        }
        println!("Program done");
    }

    fn place_holder(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let KeyModifiers::CONTROL = event.modifiers
                    && let Char(c) = event.code
                    && c == 'q'
                {
                    self.should_quit = true;
                }

                if self.should_quit {
                    break;
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
