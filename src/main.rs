use std::io::{self, Read};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    match enable_raw_mode() {
        Ok(_) => {
            for b in io::stdin().bytes() {
                let b = b.unwrap();
                let c = b as char;

                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }

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
