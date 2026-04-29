/// This mimics the current cat implementation that spits out a file into your terminal.
use std::{env::Args, fs::File, io::Read, process::exit, str::Bytes};

pub fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        let mut file = File::open(arg).expect("File should be readable");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Read to string");
    }
}

fn help() {
    println!("Usage: cat <file1>")
}
