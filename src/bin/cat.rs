use std::fs::File;
use std::io::{BufReader, Read};
/// This mimics the current cat implementation that spits out a file into your terminal.
///
/// Psuedocode (boring version):
/// Read input from command (args)
/// For each file in args
///     Open the file
///     Read the file contents
///     Print out the contents
/// exit
///
/// Pseudocode (better version):
/// Read input from command line (args)
///
///
///
use std::process::exit;

pub fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        help();
        exit(0);
    }

    for arg in args {
        let file = File::open(arg).expect("File should be readable");
        let reader = BufReader::new(file);

        let mut contents = String::new();
        for byte in reader.bytes() {
            let byte = byte.expect("Should be able to read byte");
            contents.push_str(&String::from_utf8_lossy(&[byte]));
        }

        print!("{}", contents);
    }

    exit(0)
}

fn help() {
    println!("Usage: cat <file1> ...")
}
