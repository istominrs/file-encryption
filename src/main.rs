use clap::{command, Arg};
use std::process;
use std::str::FromStr;
use std::{env, fs};
use std::path::Path;

enum Mode {
    Encrypt, // Encryption mode
    Decrypt, // Decryption mode
}

impl FromStr for Mode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Mode, Self::Err> {
        match s.to_lowercase().as_str() {
            "encryption" => Ok(Mode::Encrypt),
            "decryption" => Ok(Mode::Decrypt),
            _ => Err("invalid mode, must be 'encrypt' or 'decrypt'"),
        }
    }
}

struct Arguments {
    file_name: String,
    mode: Mode,
    key: u8,
}

impl Arguments {
    fn new() -> Result<Arguments, &'static str> {
        let args = command!()
            .arg(
                Arg::new("filename")
                    .short('f')
                    .long("file")
                    .required(true)
            )
            .arg(
                Arg::new("mode")
                    .short('m')
                    .long("mode")
                    .required(true)
            )
            .arg(
                Arg::new("key")
                    .short('k')
                    .long("key")
                    .required(true)
            )
            .get_matches();

        let file_name: String = args
            .get_one::<String>("filename")
            .ok_or("error while getting filename argument")?
            .clone();

        let mode: Mode = args
            .get_one::<String>("mode")
            .ok_or("error while getting mode argument")?
            .parse()
            .map_err(|_| "Invalid mode argument")?;

        let key: u8 = match args.get_one::<String>("key") {
            Some(k) => k
                .to_string()
                .parse()
                .map_err(|_| "error while parsing key, expected u8 number")?,
            None => return Err("error while get key argument")
        };

        Ok(Arguments {
            file_name,
            mode,
            key,
        })
    }
}

fn encryption<'a, I>(data: I, key: u8)
where
    I: Iterator<Item = &'a mut u8> {
    for d in data {
        *d ^= key;
    }
}

fn decryption<'a, I>(data: I, key: u8)
where
    I: Iterator<Item = &'a mut u8> {
    for d in data {
        *d ^= key;
    }
}

fn main() {
    let args: Arguments = match Arguments::new() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("problem parsing arguments: {}", e);
            process::exit(0);
        }
    };

    if !args.file_name.ends_with(".txt") {
        eprintln!("invalid file name. Expect *.txt");
        process::exit(1);
    }

    if !Path::new(&args.file_name).exists() {
        eprintln!("file not found.");
        process::exit(1);
    }

    let mut data = match fs::read(&args.file_name) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error while read from file: {}", e);
            process::exit(1);
        }
    };

    if data.is_empty() {
        eprintln!("input file is empty.");
        process::exit(1);
    }

    match args.mode {
        Mode::Encrypt => {
            encryption(data.iter_mut(), args.key);
        }
        Mode::Decrypt => {
            decryption(data.iter_mut(), args.key);
        }
    }

    if let Err(err) = fs::write(&args.file_name, &data) {
        eprintln!("failed to write to file: {}", err);
        process::exit(3);
    }
}
