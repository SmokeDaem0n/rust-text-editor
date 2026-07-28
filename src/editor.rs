use ::std::io;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use std::io::Read;

pub struct Editor {}

impl Editor {
    pub fn default() -> Editor {
        Editor {}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
            match b {
                Ok(b) => {
                    let c = b as char;
                    if c.is_control() {
                        println!("Binary: {b:08b} ASCII: {b:#03} \r");
                    } else {
                        println!("Binary: {b:08b} ASCII: {b:#03} Character: {c:#?}\r");
                    }

                    if c == 'q' {
                        disable_raw_mode().unwrap();
                        break;
                    }
                }
                Err(_) => {
                    println!("Error");
                }
            }
        }
        disable_raw_mode().unwrap();
    }
}
