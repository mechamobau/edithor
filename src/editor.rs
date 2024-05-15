use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::style::Color;
use crossterm::style::Colored;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        self.draw_rows();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }
    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code: Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) = event
        {
            self.should_quit = true;
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
    pub fn create_centered_message(&mut self, message: String, num_cols: u16) -> String {
        let msg_len = message.len() as u16;
        let padding = (num_cols - msg_len) / 2;
        let padding = " ".repeat(padding as usize);
        format!("{padding}{message}{padding}")
    }
    fn remove_first_char(&mut self, s: &str) -> String {
        s.chars().skip(2).collect()
    }
    fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        if let Ok((cols, rows)) = crossterm::terminal::size() {
            let middle_row = (rows as f32 / 2.0).round() as u16;

            for i in 1..rows {
                if i == middle_row {
                    let message = "edithor - version 1.0";
                    let message = self.create_centered_message(message.to_string(), cols);
                    let message = self.remove_first_char(&message);
                    println!("~{message} \r");
                } else if i == rows - 1 {
                    let message = "feito com <3 por Lucas Viana";
                    let message = self.create_centered_message(message.to_string(), cols);
                    let message = self.remove_first_char(&message);

                    println!("~{message} \r");
                } else {
                    println!("~ \r");
                }
            }
        }
        let mut stdout = stdout();
        execute!(stdout, MoveTo(0, 0))
    }
}
