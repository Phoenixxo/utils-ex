//! Counts how many words are in a file, split by spaces.
//! Outputs the word count, just as it would seem to do.
//!
//! Author: [phoenixxo](https://github.com/phoenixxo)

use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    path::PathBuf,
    process::exit,
};

pub fn main() {
    let arg = std::env::args_os().nth(1).expect(help());

    if arg.is_empty() {
        help();
        exit(0)
    }

    let path = PathBuf::from(arg);

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open {}: {}", path.display(), e);
            exit(1);
        }
    };

    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 8192];
    let mut count: u32 = 0;

    loop {
        let n = match reader.read(&mut buffer) {
            Ok(n) => n,
            Err(e) => {
                eprint!("Error occured while reading the file: {}", e);
                exit(1)
            }
        };

        if n == 0 {
            break;
        }

        let mut in_word = false;

        for byte in &buffer[..n] {
            // Checks for matches of ' ', '\n', '\t', '\r' to their ascii values
            let is_whitespace = matches!(*byte, 9..=13 | 32);
            if is_whitespace {
                in_word = false;
            } else if !in_word {
                count += 1;
                in_word = true;
            }
        }
    }

    // Take control of stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle
        .write_fmt(format_args!("WC of {:?} is {}\n", path, count))
        .unwrap();
}

fn help() -> &'static str {
    return "Usage: wc <file>";
}
