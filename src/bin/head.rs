/*!
Outputs the first 10 lines of a file by default.
Takes in either -n or -c arguments to change to output n lines or c bytes.

Author: [phoenixxo](https://github.com/phoenixxo)
*/

use std::{
    ffi::OsString,
    fs::File,
    io::{BufReader, Read, Write, stdout},
    path::PathBuf,
    process::exit,
};

use memchr::memchr_iter;

enum ArgKind {
    Lines,
    Bytes,
}

pub fn main() {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: head [-n [lines] | -c [bytes]] <file1> <file2> ...");
        exit(1);
    }

    let mut arg_kind: ArgKind = ArgKind::Lines; // Defaults to lines
    let mut output_size = 10; // Defaults to 10 lines
    let mut file_start_idx = 0; // First arg w/ file
    let mut buffer = [0u8; 8192];

    if args[0] == "-n" || args[0] == "-c" {
        if args.len() < 3 {
            eprintln!("Usage: head [-n [lines] | -c [bytes]] <file1> <file2> ...");
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

    for path in &files {
        let file = File::open(path).unwrap_or_else(|err| {
            eprintln!("Problem opening file: {:?}", err);
            exit(1);
        });

        let mut reader = BufReader::new(file);
        match arg_kind {
            ArgKind::Bytes => {
                let mut remaining_bytes: usize = output_size as usize;

                loop {
                    let n = reader
                        .read(&mut buffer[..remaining_bytes.min(0x2000)])
                        .unwrap_or_else(|err| {
                            eprintln!("Error reading file: {:?}", err);
                            exit(1);
                        });

                    if n == 0 {
                        break;
                    } // EOF

                    if n >= remaining_bytes {
                        stdout().write_all(&buffer[..remaining_bytes]).unwrap();
                        break;
                    }
                    remaining_bytes -= n;
                    stdout().write_all(&buffer[..n]).unwrap();
                }
            }
            ArgKind::Lines => {
                let mut lines_seen = 0;
                loop {
                    let n = reader.read(&mut buffer).unwrap_or_else(|err| {
                        eprintln!("Error reading file: {:?}", err);
                        exit(1);
                    });

                    if n == 0 {
                        break;
                    } // EOF

                    let chunk = &buffer[..n];
                    let mut prev = 0;

                    for pos in memchr_iter(b'\n', &chunk) {
                        lines_seen += 1;

                        if lines_seen == output_size {
                            match stdout().write_all(&chunk[prev..=pos]) {
                                Ok(it) => it,
                                Err(err) => return eprintln!("{:?}", err),
                            };
                            break;
                        }

                        prev = pos + 1;
                    }

                    if lines_seen == output_size {
                        break;
                    }

                    match stdout().write_all(&chunk[prev..]) {
                        Ok(it) => it,
                        Err(err) => return eprintln!("{:?}", err),
                    };
                }
            }
        }
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
