use std::ffi::{CString, OsString};
use std::fs::File;
use std::io::{self, Write};
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::process::exit;

struct Directory {
    name: [u8; 8],
    ext: [u8; 3],
    attr: u8,
    case: u8,
    ctime_ms: u8,
    ctime: [u8; 2],
    cdate: [u8; 2],
    adate: [u8; 2],
    start_hi: [u8; 2],
    time: [u8; 2],
    date: [u8; 2],
    start: [u8; 2],
    size: [u8; 4],
}

struct Direntry {
    dir: Box<dyn Stream>,
    entry: i32,
    dir_entry: Directory,
    name: Vec<u16>,
    begin_slot: u32,
    end_slot: u32,
}

trait Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn flush(&mut self) -> io::Result<()>;
}

struct MainParam {
    dir_callback: Option<Box<dyn FnMut(&Direntry, &mut MainParam) -> i32>>,
    mcwd: String,
}

fn mcd_callback(entry: &Direntry, mp: &mut MainParam) -> i32 {
    let path = Path::new(".mcwd");
    let mut file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("mcd: Can't open mcwd .file for writing: {}", e);
            return 16;
        }
    };

    if let Err(e) = writeln!(file, "{}", entry.name.iter().map(|&c| c as u8 as char).collect::<String>()) {
        eprintln!("Error writing to mcwd file: {}", e);
        return 16;
    }

    4 | 32
}

fn usage(ret: i32) -> ! {
    eprintln!("Mtools version {}, dated {}", env!("VERSION"), env!("DATE"));
    eprintln!("Usage: {}: [-V] [-i image] msdosdirectory", env!("PROGNAME"));
    exit(ret);
}

fn mcd(args: Vec<OsString>) {
    let mut mp = MainParam {
        dir_callback: Some(Box::new(mcd_callback)),
        mcwd: String::new(),
    };

    let mut image = None;
    let mut paths = Vec::new();

    let mut args_iter = args.into_iter().skip(1);
    while let Some(arg) = args_iter.next() {
        let arg_str = arg.to_string_lossy();
        if arg_str == "-i" {
            if let Some(img) = args_iter.next() {
                image = Some(img.into_string().unwrap());
            } else {
                usage(1);
            }
        } else if arg_str == "-h" {
            usage(0);
        } else {
            paths.push(arg_str.into_owned());
        }
    }

    if paths.len() > 1 {
        usage(1);
    }

    if paths.is_empty() {
        println!("{}", mp.mcwd);
        exit(0);
    } else {
        // Implement main_loop functionality here
        exit(0);
    }
}

fn main() {
    let args: Vec<OsString> = std::env::args_os().collect();
    mcd(args);
}