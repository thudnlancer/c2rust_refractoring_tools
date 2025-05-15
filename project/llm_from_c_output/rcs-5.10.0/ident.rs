use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader, ErrorKind};
use std::path::Path;
use std::process;

const BUFSIZ: usize = 8192;
const KDELIM: char = '$';
const VDELIM: char = '$';

struct Top;
static mut TOP: Option<Top> = None;

enum CharType {
    Letter,
    Newline,
    Unknown,
}

fn ctab(c: char) -> CharType {
    if c.is_ascii_alphabetic() {
        CharType::Letter
    } else if c == '\n' {
        CharType::Newline
    } else {
        CharType::Unknown
    }
}

struct Program {
    quiet: bool,
}

impl Program {
    fn new() -> Self {
        Program { quiet: false }
    }
}

fn match_keyword<R: Read>(fp: &mut R) -> io::Result<char> {
    let mut line = [0u8; BUFSIZ];
    let mut tp = 0;
    let mut subversion_style = false;

    let mut buf = [0; 1];
    loop {
        if fp.read_exact(&mut buf).is_err() {
            return Ok('\0');
        }
        let c = buf[0] as char;
        
        if c == VDELIM {
            break;
        }
        
        match ctab(c) {
            CharType::Letter => {
                if tp < line.len() - 4 {
                    line[tp] = c as u8;
                    tp += 1;
                } else {
                    return Ok(if c != '\0' { c } else { '\n' });
                }
            }
            _ => return Ok(if c != '\0' { c } else { '\n' }),
        }
    }

    if tp == 0 {
        return Ok(KDELIM);
    }

    line[tp] = KDELIM as u8;
    tp += 1;

    let mut buf = [0; 1];
    if fp.read_exact(&mut buf).is_err() {
        return Ok('\0');
    }
    let c = buf[0] as char;

    if c == ':' {
        subversion_style = true;
        line[tp] = c as u8;
        tp += 1;
        
        if fp.read_exact(&mut buf).is_err() {
            return Ok('\0');
        }
        let c = buf[0] as char;
    }

    if c != ' ' {
        return Ok(if c != '\0' { c } else { '\n' });
    }

    line[tp] = c as u8;
    tp += 1;

    loop {
        if fp.read_exact(&mut buf).is_err() {
            return Ok('\0');
        }
        let c = buf[0] as char;

        if c == KDELIM {
            break;
        }

        match ctab(c) {
            CharType::Newline | CharType::Unknown => {
                return Ok(if c != '\0' { c } else { '\n' });
            }
            _ => {
                if tp < line.len() - 2 {
                    line[tp] = c as u8;
                    tp += 1;
                } else {
                    return Ok(if c != '\0' { c } else { '\n' });
                }
            }
        }
    }

    if !(line[tp - 1] == b' ' || (subversion_style && line[tp - 1] == b'#')) {
        return Ok(KDELIM);
    }

    line[tp] = KDELIM as u8;
    tp += 1;
    line[tp] = b'\0';

    println!("     {}{}", KDELIM, String::from_utf8_lossy(&line[..tp-1]));
    Ok('\0')
}

fn scan_file<R: Read>(mut file: R, name: Option<&str>) -> io::Result<()> {
    if let Some(name) = name {
        println!("{}:", name);
        if io::stdout().flush().is_err() {
            return Err(io::Error::new(io::ErrorKind::Other, "write error"));
        }
    }

    let mut buf = [0; 1];
    let mut c = '\0';
    let mut quiet = false;

    loop {
        if file.read_exact(&mut buf).is_err() {
            break;
        }
        c = buf[0] as char;

        if c == KDELIM {
            match match_keyword(&mut file) {
                Ok('\0') => {
                    if io::stdout().flush().is_err() {
                        return Err(io::Error::new(io::ErrorKind::Other, "write error"));
                    }
                    quiet = true;
                }
                _ => continue,
            }
        }
    }

    if !quiet {
        eprintln!("{} warning: no id keywords in {}", 
                 name.unwrap_or("standard input"), 
                 name.unwrap_or("standard input"));
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args().skip(1);
    let mut program = Program::new();
    let mut status = process::ExitCode::SUCCESS;

    while let Some(arg) = args.next() {
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'q' => program.quiet = true,
                    'V' => {
                        if arg.len() == 2 {
                            println!("GNU RCS version info");
                            return Ok(());
                        } else {
                            eprintln!("invalid option: {}", arg);
                            return Ok(());
                        }
                    }
                    _ => {
                        eprintln!("invalid option: {}", c);
                        status = process::ExitCode::FAILURE;
                        break;
                    }
                }
            }
        } else {
            let path = Path::new(&arg);
            match File::open(path) {
                Ok(file) => {
                    if let Err(e) = scan_file(file, Some(arg.as_str())) {
                        eprintln!("{}: {}", arg, e);
                        status = process::ExitCode::FAILURE;
                    }
                    if args.len() > 0 && writeln!(io::stdout()).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("{}: {}", arg, e);
                    status = process::ExitCode::FAILURE;
                }
            }
        }
    }

    if args.len() == 0 {
        if let Err(e) = scan_file(io::stdin(), None) {
            eprintln!("standard input: {}", e);
            status = process::ExitCode::FAILURE;
        }
    }

    if io::stdout().flush().is_err() {
        eprintln!("standard output: write error");
        status = process::ExitCode::FAILURE;
    }

    if status != process::ExitCode::SUCCESS {
        process::exit(status);
    }
    Ok(())
}