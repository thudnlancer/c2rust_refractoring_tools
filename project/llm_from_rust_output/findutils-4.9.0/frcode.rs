use std::env;
use std::ffi::{CString, CStr};
use std::io::{self, Write, BufRead};
use std::process;
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int, c_long};

const SHRT_MAX: i32 = 32767;
const SHRT_MIN: i32 = -32768;

struct Option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

fn put_short(c: i32, mut writer: impl Write) -> io::Result<()> {
    assert!(c <= SHRT_MAX, "c <= SHRT_MAX");
    assert!(c >= SHRT_MIN, "c >= SHRT_MIN");
    
    writer.write_all(&[(c >> 8) as u8, c as u8])?;
    Ok(())
}

fn prefix_length(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .take_while(|(a, b)| a == b)
        .count()
}

const LONGOPTS: [Option; 4] = [
    Option {
        name: b"help\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 'h' as i32,
    },
    Option {
        name: b"version\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 'v' as i32,
    },
    Option {
        name: b"null\0".as_ptr() as *const c_char,
        has_arg: 0,
        flag: ptr::null_mut(),
        val: '0' as i32,
    },
    Option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

fn usage(status: i32) -> ! {
    if status != 0 {
        eprintln!("Try 'frcode --help' for more information.");
    } else {
        println!("Usage: frcode [-0 | --null] [--version] [--help]");
        // TODO: Implement explain_how_to_report_bugs
    }
    process::exit(status);
}

fn get_seclevel(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| {
        if s.is_empty() {
            "You need to specify a security level as a decimal integer".to_string()
        } else {
            format!("Security level {} is outside the convertible range", s)
        }
    })
}

fn outerr() -> ! {
    eprintln!("write error");
    process::exit(1);
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut path = String::new();
    let mut oldpath = String::new();
    let mut count = 0;
    let mut oldcount = 0;
    let mut delimiter = '\n';
    let mut slocate_compat = false;
    let mut slocate_seclevel = 0i64;

    // Parse command line options
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-0" | "--null" => {
                delimiter = '\0';
            }
            "-S" => {
                if i + 1 >= args.len() {
                    usage(1);
                }
                slocate_compat = true;
                slocate_seclevel = get_seclevel(&args[i+1]).unwrap_or_else(|e| {
                    eprintln!("{}", e);
                    process::exit(1);
                });
                if slocate_seclevel < 0 || slocate_seclevel > 1 {
                    eprintln!("slocate security level {} is unsupported", slocate_seclevel);
                    process::exit(1);
                }
                i += 1;
            }
            "-h" | "--help" => {
                usage(0);
            }
            "-v" | "--version" => {
                // TODO: Implement version display
                println!("frcode version");
                return Ok(());
            }
            _ => {
                usage(1);
            }
        }
        i += 1;
    }

    // Write header
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if slocate_compat {
        handle.write_all(&[if slocate_seclevel != 0 { b'1' } else { b'0' }, 0])?;
    } else {
        handle.write_all(b"\0LOCATE02\0")?;
    }

    // Process input
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let mut line = line?;
        if !line.ends_with(delimiter) {
            eprintln!("The input file should end with the delimiter");
            process::exit(1);
        }
        line.pop(); // Remove delimiter

        count = prefix_length(&oldpath, &line);
        let diffcount = count as i32 - oldcount;

        oldcount = count as i32;

        if slocate_compat {
            slocate_compat = false;
        } else {
            if diffcount < -127 || diffcount > 127 {
                handle.write_all(&[0x80])?;
                put_short(diffcount, &mut handle)?;
            } else {
                handle.write_all(&[diffcount as u8])?;
            }
        }

        handle.write_all(&line[count..].as_bytes())?;
        handle.write_all(&[0])?;

        mem::swap(&mut oldpath, &mut line);
    }

    Ok(())
}