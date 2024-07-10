// file-cryptor.exe -f -e -key <key>
// file-cryptor.exe -f -d -key <key>

use std::path::Path;
use std::process;
use std::{env, fs};

struct Arguments {
    file_name: String,

    // true for encryption,
    // false for decryption
    mode: bool,
    key: u8,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() != 6 {
            return Err("invalid number of arguments.");
        }

        let file_name = Self::get_arg_value(args, "-f")
            .ok_or("Invalid syntax. Expected -f in command arguments.")?;

        if !file_name.ends_with(".txt") {
            return Err("Invalid file name. Expected a .txt file.");
        }

        if !Path::new(&file_name).exists() {
            return Err("File not found.");
        }

        let key_str = Self::get_arg_value(args, "-key")
            .ok_or("Invalid syntax. Expected -key in command arguments.")?;

        let key: u8 = match key_str.parse() {
            Ok(k) => k,
            Err(_) => return Err("invalid key format. Expected i32."),
        };

        let mode = if args.contains(&"-e".to_string()) {
            true
        } else if args.contains(&"-d".to_string()) {
            false
        } else {
            return Err("invalid mode argument. Expected -e or -d.");
        };

        Ok(Arguments {
            file_name,
            mode,
            key,
        })
    }

    fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
        args.iter()
            .position(|x| x == flag)
            .and_then(|index| args.get(index + 1).cloned())
    }
}

fn encryption(data: &mut Vec<u8>, key: u8) {
    for d in data.iter_mut() {
        *d ^= key;
    }
}

fn decryption(data: &mut Vec<u8>, key: u8) {
    for d in data.iter_mut() {
        *d ^= key;
    }
}

fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let data: Vec<u8> = fs::read(path)?;
    Ok(data)
}

fn write_to_file(path: &str, data: Vec<u8>) -> Result<(), std::io::Error> {
    fs::write(path, data)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut data = read_file(arguments.file_name.as_str()).unwrap_or_else(|err| {
        eprintln!("problem reading data from file: {}", err);
        process::exit(2);
    });

    if data.len() == 0 {
        eprintln!("input file is empty.");
        process::exit(1);
    }

    if arguments.mode {
        encryption(&mut data, arguments.key);
        write_to_file(arguments.file_name.as_str(), data).unwrap_or_else(|err| {
            eprintln!("failed to write in file: {}", err);
            process::exit(3);
        })
    } else {
        decryption(&mut data, arguments.key);
        write_to_file(arguments.file_name.as_str(), data).unwrap_or_else(|err| {
            eprintln!("failed to write in file: {}", err);
            process::exit(3);
        })
    }
}
