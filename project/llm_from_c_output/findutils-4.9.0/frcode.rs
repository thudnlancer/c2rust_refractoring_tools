use std::cmp::min;
use std::env;
use std::ffi::CString;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::process;
use std::str;

const LOCATEDB_MAGIC: &[u8] = b"LOCATE02";
const LOCATEDB_ESCAPE: u8 = 0x80;
const LOCATEDB_ONEBYTE_MIN: i8 = -127;
const LOCATEDB_ONEBYTE_MAX: i8 = 127;

fn put_short(c: i16, mut writer: impl Write) -> io::Result<()> {
    writer.write_all(&c.to_be_bytes())?;
    Ok(())
}

fn prefix_length(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .count()
}

struct Config {
    delimiter: u8,
    slocate_compat: bool,
    slocate_seclevel: u8,
}

impl Config {
    fn from_args() -> Result<Self, String> {
        let mut args = env::args().skip(1);
        let mut delimiter = b'\n';
        let mut slocate_compat = false;
        let mut slocate_seclevel = 0;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-0" | "--null" => delimiter = 0,
                "-S" => {
                    slocate_compat = true;
                    let level = args.next().ok_or("Missing security level")?;
                    slocate_seclevel = match level.parse() {
                        Ok(0..=1) => level.parse().unwrap(),
                        _ => return Err(format!("Unsupported security level: {}", level)),
                    };
                }
                "-h" | "--help" => {
                    println!("Usage: frcode [-0 | --null] [--version] [--help]");
                    process::exit(0);
                }
                "-v" | "--version" => {
                    println!("frcode (findutils) 0.0");
                    process::exit(0);
                }
                _ => return Err(format!("Unexpected argument: {}", arg)),
            }
        }

        Ok(Config {
            delimiter,
            slocate_compat,
            slocate_seclevel,
        })
    }
}

fn main() -> io::Result<()> {
    let config = Config::from_args().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = io::stdout();

    let mut oldpath = String::new();
    let mut oldcount = 0;

    if config.slocate_compat {
        stdout.write_all(&[config.slocate_seclevel + b'0', 0])?;
    } else {
        stdout.write_all(LOCATEDB_MAGIC)?;
    }

    let mut current_line = Vec::new();
    while reader.read_until(config.delimiter, &mut current_line)? > 0 {
        if current_line.last() != Some(&config.delimiter) {
            eprintln!("Warning: Input file should end with delimiter");
        } else {
            current_line.pop(); // Remove delimiter
        }

        let path = String::from_utf8(current_line).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 in input")
        })?;

        let count = prefix_length(&oldpath, &path);
        let mut diffcount = count as i16 - oldcount as i16;

        if diffcount > i16::MAX || diffcount < i16::MIN {
            diffcount = -(oldcount as i16);
            oldcount = 0;
        } else {
            oldcount = count;
        }

        if !config.slocate_compat {
            if diffcount < LOCATEDB_ONEBYTE_MIN as i16 || diffcount > LOCATEDB_ONEBYTE_MAX as i16 {
                stdout.write_all(&[LOCATEDB_ESCAPE])?;
                put_short(diffcount, &mut stdout)?;
            } else {
                stdout.write_all(&[diffcount as u8])?;
            }
        }

        stdout.write_all(path[count..].as_bytes())?;
        stdout.write_all(&[0])?;

        oldpath = path;
        current_line.clear();
    }

    Ok(())
}