use std::fs::File;
use std::io::Read;
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

    for arg in args {
        let mut file = File::open(arg).expect("File should be readable");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Read to string");

        print!("{}", contents);
    }

    exit(0)
}

fn help() {
    println!("Usage: cat <file1>")
}
