use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    match enable_raw_mode() {
        Ok(_) => {
            for b in io::stdin().bytes() {
                let c = b.unwrap() as char;

                if c == 'q' {
                    match disable_raw_mode() {
                        Ok(_) => println!("Goodbye!"),
                        Err(e) => println!("Error disabling raw mode: {}", e),
                    }
                    break;
                }
            }
        }
        Err(e) => println!("Error enabling raw mode: {}", e),
    }
}
