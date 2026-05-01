/*!
Outputs the first 10 lines of a file by default.
Takes in either -n or -c arguments to change to output n lines or c bytes.

Author: [phoenixxo](https://github.com/phoenixxo)
*/

use std::{
    ffi::OsString,
    fs::File,
    io::{BufReader, Write, stdout},
    path::PathBuf,
    process::exit,
};

enum ArgKind {
    Lines,
    Bytes,
}

pub fn main() {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: head [-n [lines] | -c [bytes]] <file1> <file2> ...".as_bytes());
        exit(1);
    }

    let mut arg_kind: ArgKind = ArgKind::Lines; // Defaults to lines
    let mut output_size = 10; // Defaults to 10 lines
    let mut file_start_idx = 0; // First arg w/ file

    if args[0] == "-n" || args[0] == "-c" {
        if args.len() < 3 {
            eprintln!("Usage: head [-n [lines] | -c [bytes]] <file1> <file2> ...")
            exit(1);
        }

        arg_kind = match parse_arg_kind(&args[0]) {
            Ok(arg_kind) => arg_kind,
            Err(err) => {
                eprintln!("{}", err);
                exit(1);
            }
        };

        output_size = process_limit(args[1].clone());
        file_start_idx = 2;
    };

    let files: Vec<PathBuf> = args[file_start_idx..].iter().map(PathBuf::from).collect();

    for path in files {
        let file = File::open(path).unwrap_or_else(|err| {
            eprintln!("Problem opening file: {:?}", err);
            exit(1);
        });

        let reader = BufReader::new(file);
    }
}

fn process_limit(arg: OsString) -> u32 {
    let string_val = arg
        .into_string()
        .map_err(|_| "Invalid parameter input")
        .unwrap();
    string_val.trim().parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Failed to parse input of {:?}", string_val);
        exit(1);
    })
}

fn parse_arg_kind(arg: &OsString) -> Result<ArgKind, String> {
    if arg == "-n" {
        Ok(ArgKind::Lines)
    } else if arg == "-c" {
        Ok(ArgKind::Bytes)
    } else {
        Err(format!("Unknown option: {:?}", arg))
    }
}
