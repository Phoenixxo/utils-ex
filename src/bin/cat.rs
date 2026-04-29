//! Minimal implementation of `cat`.
//!
//! Writes each input file to standard output as raw bytes.
//!
//! Author: [phoenixxo](https://github.com/phoenixxo)
//!
//! # Usage
//!
//! ```text
//! cat <file1> <file2> ...
//! ```

use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::process::exit;

pub fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        help();
        exit(0);
    }

    // Reuse one fixed-size buffer so each read does not allocate.
    let mut buffer = [0u8; 8192];

    for file in args {
        let file = match File::open(file) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                continue;
            }
        };

        let mut reader = BufReader::new(file);

        loop {
            let n = match reader.read(&mut buffer) {
                Ok(buffer) => buffer,
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    break;
                }
            };

            if n == 0 {
                break;
            }

            let _ = io::stdout().write_all(&buffer[..n]);
            let _ = io::stdout().flush();
        }
    }
}

fn help() {
    println!("Usage: cat <file1> ...")
}
