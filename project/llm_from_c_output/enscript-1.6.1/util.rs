use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::collections::HashMap;
use std::os::unix::ffi::OsStrExt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::mem;
use std::ptr;
use libc::{stat, passwd, getpwuid, getuid};

const PATH_SEPARATOR: char = ':';

struct MediaEntry {
    name: String,
    w: i32,
    h: i32,
    llx: i32,
    lly: i32,
    urx: i32,
    ury: i32,
    next: Option<Box<MediaEntry>>,
}

struct FileLookupCtx {
    name: String,
    suffix: String,
    fullname: PathBuf,
}

struct InputStream {
    fp: File,
    is_pipe: bool,
    buf: Vec<u8>,
    bufpos: usize,
    data_in_buf: usize,
    nreads: usize,
    unget_ch: Vec<u8>,
    unget_pos: usize,
    unget_alloc: usize,
}

impl InputStream {
    fn open(path: &str, file: &str, input_filter: Option<&str>) -> io::Result<Self> {
        let fp = if let Some(filter) = input_filter {
            let cmd = if filter.contains("%s") {
                filter.replace("%s", file)
            } else {
                filter.to_string()
            };
            let child = Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()?;
            File::from(child.stdout.unwrap())
        } else {
            OpenOptions::new().read(true).open(Path::new(path).join(file))?
        };

        Ok(InputStream {
            fp,
            is_pipe: input_filter.is_some(),
            buf: vec![0; 4096],
            bufpos: 0,
            data_in_buf: 0,
            nreads: 0,
            unget_ch: Vec::new(),
            unget_pos: 0,
            unget_alloc: 0,
        })
    }

    fn getc(&mut self) -> io::Result<Option<u8>> {
        if self.unget_pos > 0 {
            self.unget_pos -= 1;
            return Ok(Some(self.unget_ch[self.unget_pos]));
        }

        if self.bufpos >= self.data_in_buf {
            self.data_in_buf = self.fp.read(&mut self.buf)?;
            self.bufpos = 0;
            self.nreads += 1;
            if self.data_in_buf == 0 {
                return Ok(None);
            }
        }

        let ch = self.buf[self.bufpos];
        self.bufpos += 1;
        Ok(Some(ch))
    }

    fn ungetc(&mut self, ch: u8) {
        if self.unget_pos >= self.unget_alloc {
            self.unget_alloc += 1024;
            self.unget_ch.resize(self.unget_alloc, 0);
        }
        self.unget_ch[self.unget_pos] = ch;
        self.unget_pos += 1;
    }
}

impl Drop for InputStream {
    fn drop(&mut self) {
        if self.is_pipe {
            let _ = Command::new("sh")
                .arg("-c")
                .arg("kill $PPID")
                .spawn();
        }
    }
}

fn read_config(path: &str, file: &str) -> io::Result<bool> {
    let fname = Path::new(path).join(file);
    let fp = File::open(&fname)?;
    let reader = BufReader::new(fp);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "AcceptCompositeCharacters:" => {
                if parts.len() < 2 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "missing argument"));
                }
                // accept_composites = parts[1].parse().unwrap();
            }
            // Handle other cases similarly...
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "illegal option"));
            }
        }
    }

    Ok(true)
}

fn add_media(name: String, w: i32, h: i32, llx: i32, lly: i32, urx: i32, ury: i32) -> MediaEntry {
    MediaEntry {
        name,
        w,
        h,
        llx,
        lly,
        urx,
        ury,
        next: None,
    }
}

fn file_exists(name: &str, suffix: Option<&str>) -> bool {
    let ctx = FileLookupCtx {
        name: name.to_string(),
        suffix: suffix.unwrap_or("").to_string(),
        fullname: PathBuf::new(),
    };
    pathwalk(&ctx).is_ok()
}

fn pathwalk(ctx: &FileLookupCtx) -> io::Result<bool> {
    if let Some(path) = env::var_os("PATH") {
        for path in env::split_paths(&path) {
            let full_path = path.join(&ctx.name).with_extension(&ctx.suffix);
            if full_path.exists() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn escape_string(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            '(' | ')' | '\\' => {
                result.push('\\');
                result.push(c);
            }
            _ => result.push(c),
        }
    }
    result
}

fn main() {
    // Example usage
    if let Ok(exists) = file_exists("test", Some("txt")) {
        println!("File exists: {}", exists);
    }
}